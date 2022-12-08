use axum::http::Request;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, TraceLayer},
};
use tracing::{Level, Span};

pub fn trace_log() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, TraceSpan> {
    TraceLayer::new_for_http().make_span_with(TraceSpan::new())
}

#[derive(Debug, Clone)]
pub struct TraceSpan {
    level: Level,
    include_headers: bool,
}

impl TraceSpan {
    /// Create a new `TraceSpan`.
    pub fn new() -> Self {
        Self {
            level: Level::DEBUG,
            include_headers: false,
        }
    }
}

impl Default for TraceSpan {
    fn default() -> Self {
        Self::new()
    }
}

impl<B> MakeSpan<B> for TraceSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        const TRACE_ID: &str = "X-Trace-ID";

        let trace_id = request
            .headers()
            .get(TRACE_ID)
            .map_or("random trace_id", |s| {
                s.to_str().unwrap_or("parse trace_id error")
            });
        macro_rules! make_span {
            ($level:expr) => {
                if self.include_headers {
                    tracing::span!(
                        $level,
                        "request",
                        trace_id = %trace_id,
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                        headers = ?request.headers(),
                    )
                } else {
                    tracing::span!(
                        $level,
                        "request",
                        trace_id = %trace_id,
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                }
            }
        }

        match self.level {
            Level::ERROR => {
                make_span!(Level::ERROR)
            }
            Level::WARN => {
                make_span!(Level::WARN)
            }
            Level::INFO => {
                make_span!(Level::INFO)
            }
            Level::DEBUG => {
                make_span!(Level::DEBUG)
            }
            Level::TRACE => {
                make_span!(Level::TRACE)
            }
        }
    }
}
