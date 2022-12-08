use axum::{
    extract::rejection::{FormRejection, JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use entity::orm::DbErr;
use headers::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct APIError {
    #[serde(rename(serialize = "code"))]
    pub code: u32,
    #[serde(rename(serialize = "message"))]
    pub message: String,
}

impl APIError {
    pub fn new(code: u32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
    pub fn auth_invalid(msg: String) -> Self {
        Self {
            code: 401_000,
            message: msg,
        }
    }
    pub fn service_error() -> Self {
        Self {
            code: 500_000,
            message: "Service internal error".to_owned(),
        }
    }
}

impl From<JsonRejection> for APIError {
    fn from(r: JsonRejection) -> Self {
        match r {
            JsonRejection::JsonDataError(_) => Self::new(400_000, "Param is invalid"),
            JsonRejection::JsonSyntaxError(_) => {
                Self::new(400_000, "Failed to parse the request body as JSON")
            }
            JsonRejection::MissingJsonContentType(_) => Self::new(
                400_000,
                "Json requests must have`Content-Type: application/json`",
            ),
            _ => Self::service_error(),
        }
    }
}

impl From<FormRejection> for APIError {
    fn from(r: FormRejection) -> Self {
        match r {
            FormRejection::InvalidFormContentType(_) => Self::new(
                400_000,
                "Form requests must have `Content-Type: application/x-www-form-urlencoded`",
            ),
            FormRejection::FailedToDeserializeForm(_) => {
                Self::new(400_000, "Failed to deserialize form")
            }
            _ => Self::service_error(),
        }
    }
}

impl From<DbErr> for APIError {
    fn from(r: DbErr) -> Self {
        tracing::error!("database erro: {:?}", r);
        match r {
            DbErr::ConnectionAcquire => Self::new(429_000, "Too Many Requests"),
            DbErr::RecordNotFound(_) => Self::new(0, "OK"),
            DbErr::Custom(s) => Self::new(400_000, &s),
            _ => Self::service_error(),
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let mut header = HeaderMap::new();
        header.insert(
            HeaderName::from_static("inner-status-code"),
            HeaderValue::from(self.code),
        );
        (StatusCode::OK, header, Json(self)).into_response()
    }
}
