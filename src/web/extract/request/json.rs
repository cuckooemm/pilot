use crate::web::extract::error::APIError;

use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError,
};
use serde::de::DeserializeOwned;

pub struct ReqJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ReqJson<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, APIError);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(axum::Json(value)) => Ok(Self(value)),
            Err(rejection) => Err((StatusCode::OK, APIError::from(rejection))),
        }
    }
}

impl<T> From<T> for ReqJson<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}
