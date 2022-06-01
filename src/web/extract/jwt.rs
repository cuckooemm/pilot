use crate::config;

use super::response::APIError;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use chrono::Local;
use entity::users::UserLevel;
use headers::HeaderMap;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub const AUTH_COOKIE_NAME: &str = "pilot_auth_token";

static JWT_DECODE: Lazy<DecodingKey> =
    Lazy::new(|| DecodingKey::from_secret(config::get_jwt_secret().as_bytes()));

static JWT_ENCODE: Lazy<EncodingKey> =
    Lazy::new(|| EncodingKey::from_secret(config::get_jwt_secret().as_bytes()));

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: u32,
    pub dept_id: u32,
    pub user_level: UserLevel,
    pub exp: i64,
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = (StatusCode, APIError);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let token = get_token(req.headers());
        if token.is_none() {
            return Err((
                StatusCode::OK,
                APIError::new_auth_invalid("未认证".to_owned()),
            ));
        }
        tracing::info!("token {:?}", token);
        let token_data = decode::<Claims>(&token.unwrap(), &JWT_DECODE, &Validation::default());
        if token_data.is_err() {
            tracing::error!("decode token fail. {:?}", token_data);
            return Err((
                StatusCode::OK,
                APIError::new_auth_invalid("认证失败".to_owned()),
            ));
        }
        // Ok(token_data.claims)
        Ok(token_data.unwrap().claims)
    }
}

#[inline]
pub fn auth_token(
    user_id: u32,
    dept_id: u32,
    user_level: UserLevel,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claim = Claims {
        user_id,
        dept_id,
        user_level,
        exp: Local::now().timestamp() + 86400,
    };
    encode(&Header::default(), &claim, &JWT_ENCODE)
}

pub fn set_cookie(value: &str) -> HeaderMap {
    let c = format!("{}={}", AUTH_COOKIE_NAME, value);
    let mut hm = HeaderMap::with_capacity(2);
    hm.insert(axum::http::header::SET_COOKIE, (&c).parse().unwrap());
    hm
}

fn get_token(headers: &HeaderMap) -> Option<String> {
    let token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok())
        .map(|val| val.to_string());
    match token {
        Some(token) => {
            if token.len() > 7 && token.starts_with("Bearer ") {
                return Some(token[7..].to_string());
            }
            None
        }
        None => {
            // 没有找到 authorization 尝试从 cookie 中寻找
            let cookie = headers
                .get(axum::http::header::COOKIE)
                .and_then(|val| val.to_str().ok())
                .map(|val| val.to_string());
            match cookie {
                Some(cookie) => {
                    for ck in cookie.split(';').collect::<Vec<&str>>() {
                        let item: Vec<&str> = ck.split('=').collect();
                        if item.len() != 2 {
                            continue;
                        }
                        let key = item[0].trim();
                        let token = item[1].trim();
                        if key == AUTH_COOKIE_NAME {
                            if token.len() != 0 {
                                return Some(token.to_string());
                            }
                        }
                    }
                    None
                }
                None => None,
            }
        }
    }
}
