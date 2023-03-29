use axum::http::Method;
use headers::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

pub fn cros() -> CorsLayer {
    let origin = [
        "http://localhost:3000".parse::<HeaderValue>().unwrap(),
        // "*".parse().unwrap()
    ];
    return CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::OPTIONS])
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(3600));
}
