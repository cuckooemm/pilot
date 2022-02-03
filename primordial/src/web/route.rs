use axum::{AddExtensionLayer, Router, http::StatusCode, response::{Html, IntoResponse}, routing::{get,post}};

use crate::config::StoreConfig;

use super::{backend::{api::{config},store}, user};

pub async fn init_router(conf: StoreConfig) -> Router {
    

    let config_group = Router::new()
    .route("/",get(config::get_config));

    let user_group = Router::new()
    .route("/",get(user::user));
    
    let api_group = Router::new()
    .nest("/config",config_group)
    .nest("/user",user_group);
    let db_layer = AddExtensionLayer::new(store::db::StoreStats::new(conf).await);
    Router::new()
        .route("/",get(root))
        // .fallback(not_found.into_service())
        .nest("/api",api_group)
        .layer(db_layer)
}

// basic handler that responds with a static string
async fn root() -> Html<&'static str> {
    tracing::debug!("receive request path: / ");
    Html("<h1>Hello, World!</h1>")
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}