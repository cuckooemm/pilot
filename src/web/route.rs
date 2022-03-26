use std::future::ready;

use super::{
    api::{backend::*, forent::*},
    middleware::metrics,
    store::cache::CacheItem,
    user,
};

use axum::{
    extract::Extension,
    handler::Handler,
    http::StatusCode,
    middleware,
    response::Html,
    routing::{get, post, put},
    Router,
};

pub async fn init_router() -> Router {
    let config_group = Router::new()
        .route("/desc", get(config::description))
        .route("/notifaction", get(config::notifaction));

    let user_group = Router::new().route("/", get(user::user));

    let app_group = Router::new()
        .route("/create", post(app::create))
        .route("/list", get(app::list));

    let cluster = Router::new()
        .route("/create", post(cluster::create))
        .route("/secret/reset",put(cluster::reset_secret))
        .route("/list", get(cluster::list));

    let app_ns = Router::new()
        .route("/create", post(app_extend::create))
        .route("/list", get(app_extend::list));

    let namespace = Router::new()
        .route("/create", post(namespace::create))
        .route("/list", get(namespace::list));

    let item = Router::new()
        .route("/create", post(item::create))
        .route("/list", get(item::list))
        .route("/edit", post(item::edit))
        .route("/publish/record", get(publication::publication_record))
        .route("/publish", post(item::publish))
        .route("/rollback", post(item::rollback));

    let recorder_handle = metrics::setup_metrics_recorder();

    let api_group = Router::new()
        .nest("/config", config_group)
        .nest("/user", user_group)
        .nest("/app", app_group)
        .nest("/cluster", cluster)
        .nest("/namespace", namespace)
        .nest("/app_ns", app_ns)
        .nest("/item", item);

    Router::new()
        .route("/", get(root))
        .route("/metrics", get(move || ready(recorder_handle.render())))
        .fallback(not_found.into_service())
        .nest("/api", api_group)
        // .layer(Extension(store))
        .layer(Extension(CacheItem::new()))
        .route_layer(middleware::from_fn(metrics::track_metrics))
}

// basic handler that responds with a static string
async fn root() -> Html<&'static str> {
    tracing::debug!("receive request path: / ");
    Html("<h1>Hello, World!</h1>")
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
