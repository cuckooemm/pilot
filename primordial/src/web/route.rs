use axum::{AddExtensionLayer, Router, handler::Handler, http::StatusCode, response::Html, routing::{get, post}};

use crate::config::StoreConfig;

use super::{
    backend::{
        api::{app, config},
        store,
    },
    user,
};

pub async fn init_router(conf: StoreConfig) -> Router {
    let config_group = Router::new().route("/", get(config::get_config));

    let user_group = Router::new().route("/", get(user::user));

    let app_group = Router::new()
        .route("/create", post(app::create))
        .route("/list", get(app::list));

    let api_group = Router::new()
        .nest("/config", config_group)
        .nest("/user", user_group)
        .nest("/app", app_group);

    let db_layer = AddExtensionLayer::new(store::db::StoreStats::new(conf).await);

    Router::new()
        .route("/", get(root))
        .fallback(not_found.into_service())
        .nest("/api", api_group)
        .layer(db_layer)
}

// basic handler that responds with a static string
async fn root() -> Html<&'static str> {
    tracing::debug!("receive request path: / ");
    Html("<h1>Hello, World!</h1>")
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
