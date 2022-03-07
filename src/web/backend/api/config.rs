use super::orm::EntityTrait;
use super::StoreStats;

use axum::extract::{Extension, Query};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CfgParam {
    pub app_id: Option<String>,
    pub namespace: Option<String>,
    pub secret: Option<String>,
}

pub async fn get_config(
    Query(param): Query<CfgParam>,
    Extension(store): Extension<StoreStats>,
) -> String {
    let result = super::AppEntity::find().all(&store.db).await;
    tracing::info!("receive param {:?},result {:?}", &param, &result);
    format!(
        "receive param {:?} {:?} {:?}",
        param.app_id, param.namespace, param.secret
    )
}
