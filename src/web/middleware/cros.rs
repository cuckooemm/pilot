use tower_http::cors::Any;
use axum::http::Method;
use tower_http::cors::CorsLayer;

pub fn cros() -> CorsLayer {
    let origin = [
        "http://localhost:3000".parse().unwrap(),
        // "*".parse().unwrap()
    ];
    return CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST,Method::PUT,Method::OPTIONS])
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(86400));
}
