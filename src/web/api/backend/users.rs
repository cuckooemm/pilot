use crate::web::{
    api::{check, helper},
    extract::{
        error::{APIError, ForbiddenType, ParamErrType},
        request::ReqJson,
        request::ReqQuery,
        response::APIResponse,
    },
    middleware::jwt,
    store::dao::Dao,
    APIResult,
};

use axum::{extract::State, Extension};
use chrono::Local;
use entity::{
    enums::Status,
    orm::{ActiveModelTrait, IntoActiveModel, Set},
    users::{UserItem, UserLevel},
    UsersActive, UsersModel,
};
use headers::HeaderMap;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct RegisterParam {
    pub account: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub dept_id: Option<String>,
    pub level: Option<String>,
}

// 添加用户 管理员权限
pub async fn addition(
    Extension(auth): Extension<UsersModel>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<RegisterParam>,
) -> APIResult<APIResponse<UsersModel>> {
    let account = check::account(param.account)?;
    let password = check::password(param.password)?;
    let email = check::email(param.email)?;
    let nickname = check::nickname(param.nickname)?.unwrap_or(account.clone());
    let level: UserLevel = param.level.unwrap_or_default().into();
    let mut dept_id = 0;
    if level != UserLevel::Admin {
        // 如果添加的帐号为超级管理员则无需填写部门ID
        dept_id = check::id_decode::<u32>(param.dept_id, "dept_id")?;
        if !dao.department.is_exist_id(dept_id).await? {
            return Err(APIError::param_err(ParamErrType::NotExist, "dept_id"));
        }
    }
    match auth.level {
        UserLevel::Admin => (),
        // 仅可添加本部门帐号
        UserLevel::DeptAdmin => {
            // 添加不是同一部门用户 或者 添加的帐号权限大于当前权限
            if dept_id != auth.dept_id || level > UserLevel::DeptAdmin {
                return Err(APIError::forbidden_err(ForbiddenType::Operate, "add user"));
            }
        }
        // 无权限
        _ => return Err(APIError::forbidden_err(ForbiddenType::Operate, "add user")),
    }

    if dao.users.is_exist_account_email(account.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "account"));
    }
    if dao.users.is_exist_account_email(email.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "email"));
    }
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| {
        tracing::error!("password encryption failure: {:?}", e);
        APIError::service_error()
    })?;
    let user = UsersActive {
        account: Set(account),
        email: Set(email),
        nickname: Set(nickname),
        password: Set(password),
        dept_id: Set(dept_id),
        level: Set(level),
        ..Default::default()
    };
    let data = dao.users.addition(user).await?;
    Ok(APIResponse::ok_data(data))
}

#[derive(Debug, Deserialize)]
pub struct UpdateParam {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub dept_id: Option<String>,
    pub level: Option<String>,
    pub status: Option<String>,
}

pub async fn edit(
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UsersModel>,
    ReqQuery(param): ReqQuery<UpdateParam>,
) -> APIResult<APIResponse<UsersModel>> {
    let id = match param.id {
        Some(id) => check::id_decode_rule::<u32>(&id, "id")?,
        None => auth.id,
    };

    // 找到此ID的用户信息
    let user = dao
        .users
        .get_info(id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "user"))?;

    match auth.level {
        UserLevel::Admin => (),
        UserLevel::DeptAdmin => {
            if user.dept_id != auth.dept_id {
                return Err(APIError::forbidden_resource(
                    ForbiddenType::Operate,
                    &vec!["edit", "users"],
                ));
            }
        }
        _ => {
            if user.id != auth.id {
                return Err(APIError::forbidden_resource(
                    ForbiddenType::Operate,
                    &vec!["edit", "users"],
                ));
            }
        }
    }
    let mut active = user.clone().into_active_model();

    if let Some(account) = param.account {
        if user.account != account {
            check::account_rule(&account)?;
            if dao.users.is_exist_account_email(account.clone()).await? {
                return Err(APIError::param_err(ParamErrType::Exist, "account"));
            }
            active.account = Set(account);
        }
    }
    if let Some(password) = param.password {
        check::password_rule(&password)?;
        let password = bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| {
            tracing::error!("password encryption failure: {:?}", e);
            APIError::service_error()
        })?;
        active.password = Set(password);
    }
    if let Some(email) = param.email {
        if user.email != email {
            check::email_rule(&email)?;
            if dao.users.is_exist_account_email(email.clone()).await? {
                return Err(APIError::param_err(ParamErrType::Exist, "email"));
            }
            active.email = Set(email);
        }
    }
    if let Some(nickname) = param.nickname {
        let change_nickname = check::nickname_rule(nickname)?;
        if user.nickname != change_nickname {
            active.nickname = Set(change_nickname);
        }
    }
    if let Some(dept_id) = param.dept_id {
        let dept = check::id_decode_rule::<u32>(&dept_id, "dept_id")?;
        if user.dept_id != dept {
            if auth.level != UserLevel::Admin {
                return Err(APIError::forbidden_resource(
                    ForbiddenType::Operate,
                    &vec!["change", "department"],
                ));
            }
            if !dao.department.is_exist_id(dept).await? {
                return Err(APIError::param_err(ParamErrType::NotExist, "dept_id"));
            }
            active.dept_id = Set(dept);
        }
    }
    if let Some(level) = param.level {
        let change_level = UserLevel::from(level);
        if change_level > auth.level {
            return Err(APIError::forbidden_resource(
                ForbiddenType::Operate,
                &vec!["edit", "users"],
            ));
        }
        if user.level != change_level {
            active.level = Set(change_level);
        }
    }

    if let Some(status) = param.status.and_then(|s| s.try_into().ok()) {
        if status != user.status {
            active.status = Set(status);
        }
    }

    if !active.is_changed() {
        return Ok(APIResponse::ok_data(user));
    }
    let model = dao.users.update(active).await?;
    Ok(APIResponse::ok_data(model))
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub status: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}
pub async fn list(
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UsersModel>,
    ReqQuery(param): ReqQuery<QueryParam>,
) -> APIResult<APIResponse<Vec<UserItem>>> {
    let status: Option<Status> = param.status.and_then(|s| s.try_into().ok());

    let (page, page_size) = helper::page(param.page, param.page_size);
    let mut dept = None;
    match auth.level {
        // 获取所有帐号
        UserLevel::Admin => (),
        // 获取本部门帐号
        UserLevel::DeptAdmin => dept = Some(auth.dept_id),
        // 无权限
        _ => return Err(APIError::forbidden_err(ForbiddenType::Access, "users")),
    }

    let list = dao
        .users
        .get_user_list(dept, status, helper::page_to_limit(page, page_size))
        .await?;
    let mut response = APIResponse::ok_data(list);
    response.set_page(page, page_size);
    Ok(response)
}

pub async fn register(
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<RegisterParam>,
) -> APIResult<APIResponse<()>> {
    let account = check::account(param.account)?;
    let password = check::password(param.password)?;
    let email = check::email(param.email)?;
    let nickname = check::nickname(param.nickname)?.unwrap_or(account.clone());

    // 检查帐号是否存在
    if dao.users.is_exist_account_email(account.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "account"));
    }
    if dao.users.is_exist_account_email(email.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "email"));
    }
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| {
        tracing::error!("password encryption failure: {:?}", e);
        APIError::service_error()
    })?;

    let user = UsersActive {
        account: Set(account),
        email: Set(email),
        nickname: Set(nickname),
        password: Set(password),
        ..Default::default()
    };
    dao.users.addition(user).await?;
    Ok(APIResponse::ok())
}

#[derive(Debug, Deserialize)]
pub struct LoginParam {
    pub account: Option<String>,
    pub password: Option<String>,
    pub remember: Option<bool>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub nickname: String,
    #[serde(serialize_with = "entity::confuse")]
    pub dept_id: u32,
    pub level: UserLevel,
    pub token: String,
    pub exp: i64,
}

#[instrument]
pub async fn login(
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<LoginParam>,
) -> APIResult<(HeaderMap, APIResponse<AuthResponse>)> {
    let account = check::account(param.account)?;
    let password = check::password(param.password)?;
    // 根据帐号获取信息
    let user_info = dao
        .users
        .get_info_by_account(account)
        .await?
        .ok_or(APIError::param_err(ParamErrType::Invalid, "account"))?;
    // 校验密码失败
    if !bcrypt::verify(password, &user_info.password).unwrap_or_default() {
        return Err(APIError::param_err(ParamErrType::Invalid, "password"));
    }

    // 判断帐号状态
    if user_info.status == Status::Band || user_info.status == Status::Delete {
        return Err(APIError::param_err(ParamErrType::Invalid, "account"));
    }
    let mut renewal = 3600;
    if param.remember.unwrap_or_default() {
        renewal = 86400 * 7;
    }
    let token = jwt::auth_token(user_info.id, renewal).map_err(|e| {
        tracing::error!("Token generation failure. err: {:?}", e);
        APIError::service_error()
    })?;
    let header = jwt::set_cookie(&token, param.remember.unwrap_or_default());
    // 返回session
    Ok((
        header,
        APIResponse::ok_data(AuthResponse {
            token,
            nickname: user_info.nickname,
            dept_id: user_info.dept_id,
            level: user_info.level,
            exp: Local::now().timestamp() + renewal,
        }),
    ))
}
