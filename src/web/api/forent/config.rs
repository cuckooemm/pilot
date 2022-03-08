use super::orm::EntityTrait;
use super::StoreStats;

use axum::extract::{Extension, Query};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DescParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
    pub namespace: Option<String>,
    pub secret: Option<String>,
}

// 全量获取配置数据
pub async fn description(
    Query(param): Query<DescParam>,
    Extension(store): Extension<StoreStats>,
) -> String {
    
    let result = super::AppEntity::find().all(&store.db).await;
    tracing::info!("receive param {:?},result {:?}", &param, &result);
    format!(
        "receive param {:?} {:?} {:?}",
        param.app_id, param.namespace, param.secret
    )
}

// 阻塞链接, 仅更新时返回数据
pub async fn notifaction(
    Query(param): Query<DescParam>,
    Extension(store): Extension<StoreStats>,
) -> String {
    let result = super::AppEntity::find().all(&store.db).await;
    tracing::info!("receive param {:?},result {:?}", &param, &result);
    format!(
        "receive param {:?} {:?} {:?}",
        param.app_id, param.namespace, param.secret
    )
}