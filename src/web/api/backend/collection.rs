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
use entity::model::{app::AppItem, UserAuth};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CollectionParam {
    pub app_id: Option<String>,
}

pub async fn add(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<CollectionParam>,
) -> APIResult<APIResponse<()>> {
    let app_id = check::id_str(param.app_id, "app_id")?;
    let app_id = dao
        .app
        .get_app_id(app_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "app_id"))?;
    if dao.collection.is_exist(app_id, auth.id).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "collection"));
    }
    dao.collection.addition(app_id, auth.id).await?;

    Ok(APIResponse::ok())
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub page: Option<String>,
    pub page_size: Option<String>,
}

// 获取用户收藏App
pub async fn list(
    ReqQuery(param): ReqQuery<QueryParam>,
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
) -> APIResult<APIResponse<Vec<AppItem>>> {
    let page = helper::page(param.page, param.page_size);

    let list = dao
        .collection
        .get_app(auth.id, helper::page_to_limit(page))
        .await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page);
    Ok(rsp)
}
