use std::str::FromStr;

use crate::web::api::check;
use crate::web::api::permission::accredit;
use crate::web::extract::json::ReqJson;
use crate::web::extract::jwt::Claims;
use crate::web::extract::query::ReqQuery;
use crate::web::extract::response::{APIError, APIResponse, ParamErrType};
use crate::web::store::dao::app;
use crate::web::APIResult;

use axum::extract::Json;
use entity::rule::Verb;
use entity::{AppModel, ID};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppParam {
    pub app_id: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub page: Option<String>,
    pub page_size: Option<String>,
}

// 创建APP
pub async fn create(
    ReqJson(param): ReqJson<AppParam>,
    auth: Claims,
) -> APIResult<Json<APIResponse<ID>>> {
    let app_id = check::id_str(param.app_id, "app_id")?;
    let name = match param.name {
        Some(name) => {
            if name.len() < 2 || name.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::Len(2, 100), "name"));
            }
            name
        }
        None => app_id.clone(),
    };

    if app::is_exist(app_id.clone()).await? {
        return Err(APIError::new_param_err(ParamErrType::Exist, "app_id"));
    }
    app::add(app_id, name, &auth).await?;
    Ok(Json(APIResponse::ok()))
}

// 获取用户APP
pub async fn list(
    ReqQuery(param): ReqQuery<QueryParam>,
    auth: Claims,
) -> APIResult<Json<APIResponse<Vec<AppModel>>>> {
    accredit::accredit(&auth, Verb::VIEW, vec!["some_app", "test", "namespace"]).await?;
    let (page, page_size) = check::page(param.page, param.page_size);
    let list = app::find_all((page - 1) * page_size, page_size).await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page, page_size);
    Ok(Json(rsp))
}
