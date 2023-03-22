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
use entity::{common::enums::Status, model::users::Claims};
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
            tracing::error!("failed to decode token: [{}]. err: {:?}", token, e);
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
        return Err((
            StatusCode::OK,
            APIError::auth_invalid("Authentication expired".to_owned()),
        ));
    }

    req.extensions_mut().insert(user);

    let mut response = next.run(req).await;
    if claims.exp - curtime < std::cmp::max(claims.renewal / 5, 43200) {
        // auto refresh token
        claims.exp = curtime + claims.renewal;
        encode(&Header::default(), &claims, &JWT_ENCODE)
            .and_then(|token| {
                let header = response.headers_mut();
                header.insert("New-Token", token.parse().unwrap());
                header.insert("Token-Expire", claims.exp.to_string().parse().unwrap());
                set_cookie(header, &token, claims.renewal);
                Ok(())
            })
            .map_err(|e| {
                tracing::error!("refresh Token err: {}", e);
                ()
            })
            .ok();
    }
    Ok(response)
}

#[inline]
fn get_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| {
            if let Ok(v) = v.to_str() {
                if v.len() > 7 && v.starts_with("Bearer ") {
                    return Some(v[7..].to_string());
                }
            }
            None
        })
        .or(find_cookie_token(headers))
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

fn find_cookie_token(headers: &HeaderMap) -> Option<String> {
    headers.get(axum::http::header::COOKIE).and_then(|v| {
        if let Ok(v) = v.to_str() {
            for c in v.split(';') {
                let item: Vec<&str> = c.split('=').collect();
                if item.len() != 2 {
                    continue;
                }
                let token = item[1].trim();
                if item[0].trim() == AUTH_COOKIE_NAME {
                    if token.len() != 0 {
                        return Some(token.to_string());
                    }
                }
            }
        }
        None
    })
}

pub fn set_cookie(header: &mut HeaderMap, token: &str, expires: i64) {
    let c = format!(
        "{}={}; Path=/; Expires={}",
        AUTH_COOKIE_NAME,
        token,
        (Local::now().timestamp() + expires).to_string()
    );
    header.insert(axum::http::header::SET_COOKIE, (&c).parse().unwrap());
}
