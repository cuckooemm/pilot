use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use headers::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct APIResponse<T: Serialize> {
    #[serde(rename(serialize = "code"))]
    pub code: u32,
    #[serde(rename(serialize = "message"))]
    pub message: String,
    #[serde(rename(serialize = "data"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(rename(serialize = "page"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename(serialize = "page_size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
}

impl<T> APIResponse<T>
where
    T: Serialize,
{
    #[inline]
    pub fn new(code: u32, message: String, data: Option<T>) -> Self {
        Self {
            code,
            message,
            data,
            page: None,
            page_size: None,
        }
    }
    #[inline]
    pub fn ok_data(data: T) -> Self {
        Self::new(0, "OK".to_string(), Some(data))
    }
    #[inline]
    pub fn ok() -> Self {
        Self::new(0, "OK".to_string(), None)
    }
    #[inline]
    pub fn err(code: u32, message: String) -> Self {
        Self::new(code, message, None)
    }
    pub fn set_page(&mut self, page: u64, page_size: u64) {
        self.page = Some(page);
        self.page_size = Some(page_size);
    }
}

impl<T> IntoResponse for APIResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let mut header = HeaderMap::new();
        header.insert(
            HeaderName::from_static("inner-status-code"),
            HeaderValue::from(self.code),
        );
        (StatusCode::OK, header, Json(self)).into_response()
    }
}