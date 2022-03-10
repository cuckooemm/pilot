use axum::extract::{Path, Query};
use tracing::info;

pub async fn user(Query(url): Query<String>) -> &'static str {
    info!("receive {:?}", url);
    "hello users"
}
