use crate::web::{
    api::check,
    extract::{
        json::ReqJson,
        jwt,
        response::{APIError, APIResponse, ParamErrType},
    },
    store::dao,
    APIResult,
};

use axum::Json;
use entity::{orm::Set, UsersActive, ID};
use headers::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterParam {
    pub account: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
}

pub async fn register(ReqJson(param): ReqJson<RegisterParam>) -> APIResult<Json<APIResponse<ID>>> {
    let account = check::account(param.account)?;
    let password = check::password(param.password)?;
    let email = check::email(param.email)?;
    let nickname = check::nickname(param.nickname)?.unwrap_or(account.clone());

    // 检查帐号是否存在
    let pwd = dao::users::is_exist_account_email(account.clone()).await?;
    if pwd.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "account"));
    }
    let pwd = dao::users::is_exist_account_email(email.clone()).await?;
    if pwd.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "email"));
    }
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST);
    if password.is_err() {
        tracing::error!("password encryption failure: {:?}", password);
        return Err(APIError::new_server_error());
    }
    let password = password.unwrap();
    let user = UsersActive {
        account: Set(account),
        email: Set(email),
        password: Set(password),
        nickname: Set(nickname),
        ..Default::default()
    };
    dao::users::add(user).await?;
    Ok(Json(APIResponse::ok()))
}

#[derive(Deserialize)]
pub struct LoginParam {
    pub account: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub async fn login(
    ReqJson(param): ReqJson<LoginParam>,
) -> APIResult<(HeaderMap, Json<APIResponse<AuthResponse>>)> {
    let account = check::account(param.account)?;
    let password = check::password(param.password)?;

    // 根据帐号获取信息
    let user_info = dao::users::get_info_by_account(account).await?;
    if user_info.is_none() {
        return Err(APIError::new_param_err(ParamErrType::Invalid, "account"));
    }
    let user_info = user_info.unwrap();
    // 校验密码失败
    if !bcrypt::verify(password, &user_info.password).unwrap_or_default() {
        return Err(APIError::new_param_err(ParamErrType::Invalid, "account"));
    }

    // 判断帐号状态
    if user_info.deleted_at > 0 {
        return Err(APIError::new_param_err(ParamErrType::Invalid, "account"));
    }

    // 生成认证信息
    let auth = jwt::auth_token(user_info.id, user_info.org_id, user_info.level);
    if auth.is_err() {
        tracing::error!("Token generation failure. err: {:?}", &auth);
        return Err(APIError::new_server_error());
    }
    let token = auth.unwrap();
    let header = jwt::set_cookie(&token);
    // 返回session
    Ok((header, Json(APIResponse::ok_data(AuthResponse { token }))))
}