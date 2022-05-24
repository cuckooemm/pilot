// use axum::{http::Request, middleware::Next, response::IntoResponse, Json, Extension};
// use entity::ID;
// use headers::HeaderMap;
// use jsonwebtoken::{decode, DecodingKey, Validation};

// use crate::web::extract::{response::APIResponse, jwt_util};

// pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
//     let token = get_token(req.headers());
//     if token.is_none() {
//         return Json(APIResponse::<ID>::new(4100,"未认证".to_owned(),None)).into_response();
//     }
//     tracing::info!("token {:?}", token);
//     let token_data = decode::<jwt_util::Claims>(
//         &token.unwrap(),
//         &DecodingKey::from_secret(jwt_util::JWT_KEY),
//         &Validation::default(),
//     );
//     if token_data.is_err() {
//         tracing::error!("decode token fail. {:?}", token_data);
//         return Json(APIResponse::<ID>::new(4100,"认证失败".to_owned(),None)).into_response();
//     }
//     req.extensions_mut().insert(Extension(token_data.unwrap()));
//     let response = next.run(req).await;
//     response
// }
