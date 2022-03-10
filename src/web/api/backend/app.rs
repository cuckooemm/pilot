use super::orm::Set;
use super::response::APIResponse;
use super::ReqJson;
use super::{check, APIResult, AppActive, AppModel};

use axum::extract::Json;
use entity::dao::app;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppParam {
    pub app_id: Option<String>,
    pub name: Option<String>,
}

// 创建APP
pub async fn create(ReqJson(param): ReqJson<AppParam>) -> APIResult<Json<APIResponse<AppModel>>> {
    let app_id = check::app_id(param.app_id)?;
    let name = check::name(param.name, "name")?;
    let data = AppActive {
        app_id: Set(app_id),
        name: Set(name),
        // TODO 填充其他信息
        ..Default::default()
    };
    let result = app::insert_one(data).await?;
    tracing::info!("{:?}", &result);
    Ok(Json(APIResponse::ok(Some(result))))
}

// 获取所有APP
pub async fn list() -> APIResult<Json<APIResponse<Vec<AppModel>>>> {
    let list = app::find_all().await?;
    Ok(Json(APIResponse::ok(Some(list))))
}
