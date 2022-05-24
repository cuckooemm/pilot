use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use serde::de::DeserializeOwned;
use std::ops::Deref;

use super::response::{APIError, APIErrorType};

#[cfg_attr(docsrs, doc(cfg(feature = "query")))]
#[derive(Debug, Clone, Copy, Default)]
pub struct ReqQuery<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ReqQuery<T>
where
    T: DeserializeOwned,
    B: Send,
{
    type Rejection = (StatusCode, APIError);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let query = req.uri().query().unwrap_or_default();
        match serde_urlencoded::from_str(query) {
            Ok(value) => Ok(ReqQuery(value)),
            Err(_) => Err((
                StatusCode::OK,
                APIError::with_param(
                    APIErrorType::BadRequestBody,
                    Some("invalid path param".to_owned()),
                ),
            )),
        }
    }
}

impl<T> Deref for ReqQuery<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
