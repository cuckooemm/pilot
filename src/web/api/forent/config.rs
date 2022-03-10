use axum::extract::Query;
use entity::orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DescParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
    pub namespace: Option<Vec<String>>,
    pub secret: Option<String>,
}

// 全量获取配置数据
pub async fn description(Query(param): Query<DescParam>) -> String {
    // 校验 appid cluster 是否存
    
    // let result = entity::AppEntity::find().all(&store.db).await;
    // tracing::info!("receive param {:?},result {:?}", &param, &result);
    format!(
        "receive param {:?} {:?} {:?}",
        param.app_id, param.namespace, param.secret
    )
}

// 阻塞链接, 仅更新时返回数据
pub async fn notifaction(Query(param): Query<DescParam>) -> String {
    // let result = entity::AppEntity::find().all(&store.db).await;
    // tracing::info!("receive param {:?},result {:?}", &param, &result);
    format!(
        "receive param {:?} {:?} {:?}",
        param.app_id, param.namespace, param.secret
    )
}
