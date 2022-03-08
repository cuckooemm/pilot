use super::response::APIError;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    BoxError,
};
use serde::de::DeserializeOwned;

pub struct ReqJson<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for ReqJson<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::Json`
    T: DeserializeOwned,
    B: axum::body::HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, APIError);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err((StatusCode::OK, APIError::new_parse_err(rejection))),
        }
    }
}
