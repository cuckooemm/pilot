use crate::web::{
    api::{check, helper},
    extract::{
        error::{APIError, ParamErrType},
        request::{ReqJson, ReqQuery},
        response::APIResponse,
    },
    store::dao::Dao,
    APIResult,
};

use axum::{extract::State, Extension};
use entity::{
    common::enums::Status,
    model::{AppModel, CollectionActive, UserAuth},
    orm::{Set, IntoActiveModel},
};
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct Param {
    pub app: Option<String>,
    pub cancel: Option<bool>,
}

#[instrument(skip(dao, auth))]
pub async fn add(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<Param>,
) -> APIResult<APIResponse<()>> {
    let app = check::id_str(param.app, "app")?;
    let app = dao
        .app
        .get_info(app)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "app"))?;
    match app.status {
        Status::Band => return Err(APIError::param_err(ParamErrType::Invalid, "app")),
        Status::Delete => return Err(APIError::param_err(ParamErrType::NotExist, "app")),
        _ => (),
    }
    let cancel = param.cancel.unwrap_or_default();
    match dao.collection.get_collection(app.id, auth.id).await? {
        Some(model) => {
            let mut active = model.clone().into_active_model();
            if cancel {
                if model.status == Status::Delete {
                    return Err(APIError::param_err(ParamErrType::NotExist, "collection"));
                }
                active.status = Set(Status::Delete);
            } else {
                if model.status == Status::Normal {
                    return Err(APIError::param_err(ParamErrType::Exist, "collection"));
                }
                active.status = Set(Status::Normal);
            }
            dao.collection.save(active).await?;
        }
        None => {
            if cancel {
                return Err(APIError::param_err(ParamErrType::NotExist, "collection"));
            }
            dao.collection.addition(app.id, auth.id).await?;
        }
    }
    Ok(APIResponse::ok())
}

#[derive(Deserialize, Debug)]
pub struct QueryParam {
    pub status: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn list(
    ReqQuery(param): ReqQuery<QueryParam>,
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
) -> APIResult<APIResponse<Vec<AppModel>>> {
    let page = helper::page(param.page, param.page_size);
    let status: Option<Status> = param.status.and_then(|x| x.try_into().ok());
    let list = dao
        .collection
        .get_apps(auth.id, status, helper::page_to_limit(page))
        .await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page);
    Ok(rsp)
}
