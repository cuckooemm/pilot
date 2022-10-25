use std::future::ready;

use super::{
    api::{backend::*, forent::*},
    middleware::{cros, metrics},
    store::cache::CacheItem,
};

use axum::{
    extract::Extension,
    handler::Handler,
    http::StatusCode,
    middleware,
    routing::{get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub async fn init_router() -> Router {
    let config_group = Router::new()
        .route("/desc", get(config::description))
        .route("/notifaction", get(config::notifaction));

    let users_group = Router::new()
        .route("/register", post(users::register))
        .route("/login", post(users::login))
        .route("/addition", post(users::addition))
        .route("/list", get(users::list))
        .route("/edit", put(users::edit));

    let department_group = Router::new()
        .route("/create", post(department::create))
        .route("/edit", put(department::edit))
        .route("/list", get(department::list));

    let app_group = Router::new()
        .route("/create", post(app::create))
        .route("/edit", put(app::edit))
        .route("/list", get(app::list))
        .route("/favorite", get(favorite::list))
        .route("/favorite/add", post(favorite::add));

    let cluster = Router::new()
        .route("/create", post(cluster::create))
        .route("/secret/reset", put(cluster::reset_secret))
        .route("/list", get(cluster::list));

    let app_extend = Router::new()
        .route("/create", post(app_extend::create))
        .route("/list", get(app_extend::list));

    let namespace = Router::new()
        .route("/create", post(namespace::create))
        .route("/list", get(namespace::list))
        .route("/public", get(namespace::list_public));

    let item = Router::new()
        .route("/create", post(item::create))
        .route("/list", get(item::list))
        .route("/edit", put(item::edit))
        .route("/publish/history", get(publication::release_list))
        .route("/publish", post(publication::publish))
        .route("/rollback", post(publication::rollback));

    let api_group = Router::new()
        .nest("/config", config_group)
        .nest("/app", app_group)
        .nest("/department", department_group)
        .nest("/users", users_group)
        .nest("/cluster", cluster)
        .nest("/namespace", namespace)
        .nest("/app_extend", app_extend)
        .nest("/item", item);

    let mid = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cros::cros())
        .layer(middleware::from_fn(metrics::track_metrics))
        .into_inner();
    let recorder_handle = metrics::setup_metrics_recorder();
    Router::new()
        .route("/metrics", get(move || ready(recorder_handle.render())))
        .fallback(not_found.into_service())
        .nest("/api", api_group)
        // .layer(Extension(store))
        .layer(Extension(CacheItem::new()))
        .layer(mid)
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
