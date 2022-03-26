use super::dao::app;
use super::response::{APIError, APIResponse, ParamErrType};
use super::{check, APIResult};
use super::{ReqJson, ReqQuery};

use axum::extract::Json;
use entity::orm::Set;
use entity::{AppActive, AppModel, ID};
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
pub async fn create(ReqJson(param): ReqJson<AppParam>) -> APIResult<Json<APIResponse<ID>>> {
    let app_id = check::app_id(param.app_id)?;
    let name = check::name(param.name, "name")?;
    let record = app::is_exist(app_id.clone()).await?;
    if record.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "app_id"));
    }

    let data = AppActive {
        app_id: Set(app_id),
        name: Set(name),
        // TODO 填充其他信息
        ..Default::default()
    };
    let id = app::insert(data).await?;
    Ok(Json(APIResponse::ok_data(ID::new(id))))
}

// 获取所有APP
pub async fn list(
    ReqQuery(param): ReqQuery<QueryParam>,
) -> APIResult<Json<APIResponse<Vec<AppModel>>>> {
    let (page, page_size) = check::page(param.page, param.page_size);
    let list = app::find_all((page - 1) * page_size, page_size).await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page, page_size);
    Ok(Json(rsp))
}
