use crate::{
    config,
    web::{extract::error::APIError, store::dao::users},
};

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use chrono::Local;
use entity::model::{
    enums::Status,
    users::{Claims, UserLevel},
};
use headers::HeaderMap;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;

pub const AUTH_COOKIE_NAME: &str = "pilot_auth_token";

static JWT_ENCODE: Lazy<EncodingKey> =
    Lazy::new(|| EncodingKey::from_secret(config::get_jwt_secret().as_bytes()));

static JWT_DECODE: Lazy<DecodingKey> =
    Lazy::new(|| DecodingKey::from_secret(config::get_jwt_secret().as_bytes()));

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let token = get_token(req.headers()).ok_or((
        StatusCode::OK,
        APIError::auth_invalid("Authentication failed".to_owned()),
    ))?;
    let mut claims = decode::<Claims>(&token, &JWT_DECODE, &Validation::default())
        .map_err(|e| {
            tracing::error!("failed to decode token [{}]. err: {:?}", token, e);
            (
                StatusCode::OK,
                APIError::auth_invalid("Authentication failed".to_owned()),
            )
        })?
        .claims;
    let user = users::Users
        .get_user_info(claims.uid)
        .await
        .unwrap_or_default()
        .ok_or((
            StatusCode::OK,
            APIError::auth_invalid("Authentication failed".to_owned()),
        ))?;
    if user.status != Status::Normal {
        return Err((
            StatusCode::OK,
            APIError::auth_invalid("Invalid account".to_owned()),
        ));
    }
    let curtime = Local::now().timestamp();
    if claims.exp < curtime {
        // expire
        return Err((
            StatusCode::OK,
            APIError::auth_invalid("Authentication expired".to_owned()),
        ));
    }

    req.extensions_mut().insert(user);

    let mut response = next.run(req).await;
    if claims.renewal > 0 && claims.exp - curtime < std::cmp::max(claims.renewal / 5, 86400) {
        // auto refresh token
        claims.exp = curtime + claims.renewal;
        encode(&Header::default(), &claims, &JWT_ENCODE)
            .and_then(|token| {
                let header = response.headers_mut();
                header.insert("Refresh-Token", token.parse().unwrap());
                header.insert("Token-Expire", claims.exp.to_string().parse().unwrap());
                Ok(())
            })
            .ok();
    }
    Ok(response)
}

#[inline]
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

#[inline]
pub fn auth_token(user_id: u32, renewal: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let claim = Claims {
        uid: user_id,
        renewal,
        exp: Local::now().timestamp() + renewal,
    };
    encode(&Header::default(), &claim, &JWT_ENCODE)
}

pub fn set_cookie(value: &str, remember: bool) -> HeaderMap {
    let mut c = format!("{}={}", AUTH_COOKIE_NAME, value);
    c.push_str("; Path=/");
    if remember {
        c.push_str(&format!(
            "; Expires={}",
            (Local::now().timestamp() + (86400_i64 * 3)).to_string()
        ))
    }
    let mut hm = HeaderMap::with_capacity(2);
    hm.insert(axum::http::header::SET_COOKIE, (&c).parse().unwrap());
    hm
}
