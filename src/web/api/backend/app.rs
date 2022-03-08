use super::orm::{ActiveModelTrait, EntityTrait, Set};
use super::response::APIResponse;
use super::{check, APIResult, AppActive, AppEntity, AppModel};
use super::{ReqJson, StoreStats};

use axum::extract::{Extension, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppParam {
    pub app_id: Option<String>,
    pub name: Option<String>,
}

// 创建APP
pub async fn create(
    ReqJson(param): ReqJson<AppParam>,
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<AppModel>>> {
    let app_id = check::app_id(param.app_id)?;
    let name = check::name(param.name, "name")?;
    let data = AppActive {
        app_id: Set(app_id),
        name: Set(name),
        // TODO 填充其他信息
        ..Default::default()
    };
    let result = data.insert(&store.db).await?;
    tracing::info!("{:?}", &result);
    Ok(Json(APIResponse::ok(Some(result))))
}

// 获取所有APP
pub async fn list(
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<Vec<AppModel>>>> {
    let list: Vec<AppModel> = AppEntity::find().all(&store.db).await?;
    Ok(Json(APIResponse::ok(Some(list))))
}
