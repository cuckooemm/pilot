use axum::{
    async_trait,
    body::HttpBody,
    extract::{
        rejection::{FormRejection, JsonRejection},
        FromRequest,
    },
    http::{header::CONTENT_TYPE, Request},
    BoxError, Form, Json,
};

use crate::web::extract::error::error::APIError;

pub enum JsonOrForm<T> {
    Json(T),
    Form(T),
}

#[async_trait]
impl<S, B, T> FromRequest<S, B> for JsonOrForm<T>
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    T: 'static,
{
    type Rejection = APIError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let result = match axum::Json::<T>::from_request(req, state).await {
                    Ok(Json(value)) => Ok(Self::Json(value)),
                    Err(rejection) => Err(rejection.into()),
                };
                return result;
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let result = match axum::Form::<T>::from_request(req, state).await {
                    Ok(Form(value)) => Ok(Self::Form(value)),
                    Err(rejection) => Err(rejection.into()),
                };
                return result;
            }
        }

        Err(APIError {
            code: 4000,
            message: "requests must have `Content-Type".to_owned(),
        })
    }
}
