use super::{check, APIResult};
use crate::web::api::helper;
use crate::web::api::permission::accredit;
use crate::web::extract::error::{APIError, ForbiddenType, ParamErrType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::Dao;

use axum::extract::State;
use axum::Extension;
use entity::orm::Set;
use entity::rule::Verb;
use entity::{AppExtraActive, NamespaceModel, Scope, UserAuth};
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct AppExtraParam {
    pub app: Option<String>,
    pub namespace_id: Option<String>,
    pub cancel: Option<bool>,
}

#[instrument(skip(dao, auth))]
pub async fn create(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<AppExtraParam>,
) -> APIResult<APIResponse<()>> {
    let app = check::id_str(param.app, "app")?;
    let namespace_id = check::id_decode(param.namespace_id, "namespace_id")?;
    if !dao.app.is_exist(app.clone()).await? {
        return Err(APIError::param_err(ParamErrType::NotExist, "app"));
    }
    let resource = vec![app.as_str()];
    if !accredit::accredit(&auth, Verb::Modify, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Operate,
            &resource,
        ));
    }

    let namespace = dao
        .namespace
        .get_namespace_by_id(namespace_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "namespace_id"))?;
    if namespace.app.eq(&app) {
        return Err(APIError::param_err(ParamErrType::Invalid, "namespace_id"));
    }
    if namespace.scope != Scope::Public {
        return Err(APIError::param_err(ParamErrType::Invalid, "namespace_id"));
    }
    if param.cancel.unwrap_or_default() {
        if !dao.app_extra.is_exist(app.clone(), namespace.id).await? {
            return Err(APIError::param_err(ParamErrType::NotExist, "namespace_id"));
        }
        dao.app_extra.deleted(app, namespace.id).await?;
    } else {
        if dao.app_extra.is_exist(app.clone(), namespace.id).await? {
            return Err(APIError::param_err(ParamErrType::Exist, "namespace_id"));
        }
        let data = AppExtraActive {
            app: Set(app),
            namespace_id: Set(namespace.id),
            ..Default::default()
        };
        dao.app_extra.addition(data).await?;
    }

    Ok(APIResponse::ok())
}

#[derive(Deserialize, Debug)]
pub struct QueryParam {
    pub app: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn list(
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
    ReqQuery(param): ReqQuery<QueryParam>,
) -> APIResult<APIResponse<Vec<NamespaceModel>>> {
    let app = check::id_str(param.app, "app")?;
    let (page, page_size) = helper::page(param.page, param.page_size);
    let resource = vec![app.as_str()];
    if !accredit::accredit(&auth, entity::rule::Verb::VIEW, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Access,
            &resource,
        ));
    }
    let list: Vec<NamespaceModel> = dao
        .app_extra
        .get_app_namespace(app, helper::page_to_limit(page, page_size))
        .await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page, page_size);
    Ok(rsp)
}
