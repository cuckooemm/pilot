use super::super::error::APIError;

use serde::de::DeserializeOwned;
use std::ops::Deref;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct ReqQuery<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for ReqQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, APIError);
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        match serde_urlencoded::from_str(query) {
            Ok(v) => Ok(ReqQuery(v)),
            Err(_) => Err((StatusCode::OK, APIError::new(400000, "Invalid path param"))),
        }
    }
}

impl<T> Deref for ReqQuery<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
