use axum::{
    http::{header::HeaderName, Request},
    middleware::Next,
    response::IntoResponse,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use tokio::time::Instant;

pub fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .set_buckets_for_metric(
            Matcher::Full("database_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

pub async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = req.uri().path().to_owned();
    let method = req.method().clone();
    let response = next.run(req).await;
    let latency = start.elapsed().as_secs_f64();
    let code: String;
    let status = response.status().as_u16();
    if status == 200 {
        code = if let Some(v) = response
            .headers()
            .get(HeaderName::from_static("inner-status-code"))
        {
            v.to_str().unwrap_or("500").to_owned()
        } else {
            "0".to_owned()
        };
    } else {
        code = status.to_string();
    }

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("code", code),
    ];
    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}
