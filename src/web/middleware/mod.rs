use tower::ServiceBuilder;

pub mod cros;
pub mod jwt;
pub mod metrics;

pub fn middleware() -> ServiceBuilder<
    tower::layer::util::Stack<tower_http::cors::CorsLayer, tower::layer::util::Identity>,
> {
    return ServiceBuilder::new().layer(cros::cros());
}
