use std::future::ready;

use super::{
    api::{backend::*, forent::*},
    middleware::{cros, jwt::auth, metrics, trace},
    store::Store,
};

use axum::{
    http::StatusCode,
    middleware,
    routing::{get, post, put},
    Router,
};
use tower::ServiceBuilder;

pub async fn init_router() -> Router {
    let account_group = Router::new()
        .route("/register", post(users::register))
        .route("/login", post(users::login));

    let config_router = Router::new()
        .route("/desc", get(config::description))
        .route("/notifaction", get(config::notifaction));

    let api_group = Router::new()
        .nest("/config", config_router)
        .nest("/auth", account_group)
        .merge(auth_router());

    let layer = ServiceBuilder::new()
        .layer(trace::trace_log())
        .layer(cros::cros())
        .layer(middleware::from_fn(metrics::track_metrics))
        .into_inner();

    let metrics_handle = metrics::setup_metrics_recorder();
    Router::new()
        .nest("/api", api_group)
        .route("/metrics", get(move || ready(metrics_handle.render())))
        .with_state(Store::new().await)
        // .layer(Extension(store))
        // .with_state(CacheItem::new())
        .layer(layer)
        .fallback(not_found)
}

fn auth_router() -> Router<Store> {
    let users_group = Router::new()
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
        .route("/collection", get(collection::list))
        .route("/collection/add", post(collection::add));

    let cluster = Router::new()
        .route("/create", post(cluster::create))
        .route("/edit", put(cluster::edit))
        .route("/list", get(cluster::list));

    let app_extend = Router::new()
        .route("/create", post(app_extra::create))
        .route("/list", get(app_extra::list));

    let namespace = Router::new()
        .route("/create", post(namespace::create))
        .route("/edit", put(namespace::edit))
        .route("/list", get(namespace::list))
        .route("/public", get(namespace::list_public));

    let item = Router::new()
        .route("/create", post(item::create))
        .route("/list", get(item::list))
        .route("/edit", put(item::edit))
        .route("/history/list", get(item::history_list))
        .route("/publish/history", get(publication::list))
        .route("/publish", post(publication::publish))
        .route("/rollback", post(publication::rollback));

    Router::new()
        .nest("/app", app_group)
        .nest("/cluster", cluster)
        .nest("/namespace", namespace)
        .nest("/app_extend", app_extend)
        .nest("/users", users_group)
        .nest("/department", department_group)
        .nest("/item", item)
        .layer(middleware::from_fn(auth))
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
