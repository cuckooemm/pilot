use axum::extract::{Extension, Json};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;

use crate::web::backend::store::db::StoreStats;

use super::response::APIResponse;
use super::{Application, ApplicationActive, ApplicationModel,APIResult};

#[derive(Deserialize)]
pub struct AppParam {
    pub name: String,
}

// 创建APP
pub async fn create(
    Json(param): Json<AppParam>,
    Extension(store): Extension<StoreStats>,
) -> String {
    let data = ApplicationActive {
        app_id: Set(param.name.to_owned()),
        name: Set(param.name.to_owned()),
        ..Default::default()
    };
    let result = data.insert(&store.db).await;
    tracing::info!("{:?}", result);
    param.name
}   

// 获取所有APP
pub async fn list(
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<Vec<ApplicationModel>>>> {
    let list: Vec<ApplicationModel> = Application::find().all(&store.db).await?;
    Ok(Json(APIResponse::ok(list)))
}
