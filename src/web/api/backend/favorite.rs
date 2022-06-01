use crate::web::{
    api::check,
    extract::{
        json::ReqJson,
        jwt::Claims,
        query::ReqQuery,
        response::{APIError, ApiResponse, ParamErrType},
    },
    store::dao::{app, favorite},
    APIResult,
};

use axum::Json;
use entity::{app::AppItem, ID};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FavoriteParam {
    pub app_id: Option<String>,
}

pub async fn add(
    ReqJson(param): ReqJson<FavoriteParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<ID>>> {
    let app_id = check::id_str(param.app_id, "app_id")?;

    // 查看 app_id 是否存在
    let app_id = app::get_app_id(app_id).await?;
    if app_id.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
    }
    let app_id = app_id.unwrap();
    if favorite::is_exist(app_id, auth.user_id).await? {
        return Err(APIError::new_param_err(ParamErrType::Exist, "app_id"));
    }
    // 查看是否已经存在此收藏记录
    favorite::add(app_id, auth.user_id).await?;

    Ok(Json(ApiResponse::ok()))
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub page: Option<String>,
    pub page_size: Option<String>,
}

// 获取用户收藏App
pub async fn list(
    ReqQuery(param): ReqQuery<QueryParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<Vec<AppItem>>>> {
    let (page, page_size) = check::page(param.page, param.page_size);

    let list = favorite::get_app(auth.user_id, (page - 1) * page_size, page_size).await?;
    let mut rsp = ApiResponse::ok_data(list);
    rsp.set_page(page, page_size);
    Ok(Json(rsp))
}
