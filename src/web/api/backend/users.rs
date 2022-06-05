use crate::web::{
    api::check,
    extract::{
        json::ReqJson,
        jwt::{self, Claims},
        query::ReqQuery,
        response::{APIError, ApiResponse, Empty, ParamErrType},
    },
    store::dao::{self, department, users},
    APIResult,
};

use axum::Json;
use chrono::Local;
use entity::{
    common::Status,
    orm::{ActiveModelTrait, IntoActiveModel, Set},
    users::{UserItem, UserLevel},
    UsersActive, UsersModel, ID,
};
use headers::HeaderMap;
use serde::{Deserialize, Serialize};

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
    ReqJson(param): ReqJson<RegisterParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<Empty>>> {
    let account = check::account(param.account)?;
    let password = check::password(param.password)?;
    let email = check::email(param.email)?;
    let nickname = check::nickname(param.nickname)?.unwrap_or(account.clone());
    let level: UserLevel = param.level.unwrap_or_default().into();
    let mut dept_id = 0;
    if level != UserLevel::Admin {
        // 如果添加的帐号为超级管理员则无需填写部门ID
        dept_id = check::id_decode::<u32>(param.dept_id, "dept_id")?;
        if !department::is_exist_id(dept_id).await? {
            return Err(APIError::new_param_err(ParamErrType::NotExist, "dept_id"));
        }
    }
    match auth.user_level {
        UserLevel::Admin => (),
        // 仅可添加本部门帐号
        UserLevel::DeptAdmin => {
            // 添加不是同一部门用户 或者 添加的帐号权限大于当前权限
            if dept_id != auth.dept_id || level > UserLevel::DeptAdmin {
                return Err(APIError::new_permission_forbidden());
            }
        }
        // 无权限
        UserLevel::Normal => return Err(APIError::new_permission_forbidden()),
    }

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
    let user = UsersActive {
        account: Set(account),
        email: Set(email),
        password: Set(password.unwrap()),
        nickname: Set(nickname),
        dept_id: Set(dept_id),
        level: Set(level),
        ..Default::default()
    };
    dao::users::add(user).await?;
    Ok(Json(ApiResponse::ok()))
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
    ReqJson(param): ReqJson<UpdateParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<UsersModel>>> {
    let id = check::id_decode::<u32>(param.id, "id")?;

    // 找到此ID的用户信息
    let user = users::get_info(id).await?;
    if user.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    let user = user.unwrap();

    let account = match param.account {
        Some(account) => {
            check::account_rule(&account)?;
            if user.account != account {
                Some(account)
            } else {
                None
            }
        }
        None => None,
    };
    let password = match param.password {
        Some(password) => {
            check::password_rule(&password)?;
            Some(password)
        }
        None => None,
    };
    let email = match param.email {
        Some(email) => {
            check::email_rule(&email)?;
            if user.email != email {
                Some(email)
            } else {
                None
            }
        }
        None => None,
    };
    let nickname = match param.nickname {
        Some(nickname) => {
            let nm = check::nickname_rule(nickname)?;
            if user.nickname != nm {
                Some(nm)
            } else {
                None
            }
        }
        None => None,
    };
    let dept_id = match param.dept_id {
        Some(dept) => {
            let dept = check::id_decode_rule::<u32>(&dept, "dept_id")?;
            if user.dept_id != dept {
                // 检查部门ID是否存在
                if !department::is_exist_id(dept).await? {
                    return Err(APIError::new_param_err(ParamErrType::NotExist, "dept_id"));
                }
                Some(dept)
            } else {
                None
            }
        }
        None => None,
    };
    let level = match param.level {
        Some(level) => {
            let level = UserLevel::from(level);
            if user.level != level {
                Some(level)
            } else {
                None
            }
        }
        None => None,
    };
    let delete = match param.status {
        Some(s) => match Status::from(s) {
            Status::Other => None,
            Status::Normal => {
                // 本身状态正常 无需更改
                if user.deleted_at == 0 {
                    None
                } else {
                    // 状态为delete 更改为正常
                    Some(0)
                }
            }
            Status::Delete => {
                if user.deleted_at != 0 {
                    None
                } else {
                    Some(Local::now().timestamp() as u64)
                }
            }
        },
        None => None,
    };

    match auth.user_level {
        UserLevel::Admin => (),
        // 仅可添加本部门帐号
        UserLevel::DeptAdmin => {
            // 判断帐号是否同属于一个部门
            if user.dept_id != auth.dept_id {
                return Err(APIError::new_permission_forbidden());
            }
            // 判断修改的账户等级是否大于当前账户权限等级
            if let Some(level) = level {
                if level > UserLevel::DeptAdmin {
                    return Err(APIError::new_permission_forbidden());
                }
            }
            // 阻止跨部门的修改
            if let Some(id) = dept_id {
                if id != auth.dept_id {
                    return Err(APIError::new_permission_forbidden());
                }
            }
        }
        // 无权限
        UserLevel::Normal => return Err(APIError::new_permission_forbidden()),
    }

    let mut active = user.clone().into_active_model();

    // 检查帐号是否存在 并更新
    if let Some(account) = account {
        let id = dao::users::is_exist_account_email(account.clone()).await?;
        if id.is_some() {
            return Err(APIError::new_param_err(ParamErrType::Exist, "account"));
        }
        active.account = Set(account);
    }
    // 检查 email 是否存在 并更新
    if let Some(email) = email {
        let id = dao::users::is_exist_account_email(email.clone()).await?;
        if id.is_some() {
            return Err(APIError::new_param_err(ParamErrType::Exist, "email"));
        }
        active.email = Set(email);
    }
    // 检查部门是否存在 并更新
    if let Some(dept_id) = dept_id {
        if !department::is_exist_id(dept_id).await? {
            return Err(APIError::new_param_err(ParamErrType::NotExist, "email"));
        }
        active.dept_id = Set(dept_id);
    }

    // 更新密码
    if let Some(password) = password {
        let password = bcrypt::hash(password, bcrypt::DEFAULT_COST);
        if password.is_err() {
            tracing::error!("password encryption failure: {:?}", password);
            return Err(APIError::new_server_error());
        }
        active.password = Set(password.unwrap())
    }
    // 更新 nickname
    if let Some(nickname) = nickname {
        active.nickname = Set(nickname);
    }
    if let Some(level) = level {
        active.level = Set(level);
    }
    if let Some(delete) = delete {
        active.deleted_at = Set(delete);
    }
    if !active.is_changed() {
        return Ok(Json(ApiResponse::ok_data(user)));
    }
    let model = users::update(active).await?;
    Ok(Json(ApiResponse::ok_data(model)))
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub status: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}
pub async fn list(
    ReqQuery(param): ReqQuery<QueryParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<Vec<UserItem>>>> {
    // 默认获取状态正常用户
    let status: Status = param.status.unwrap_or_default().into();

    let (page, page_size) = check::page(param.page, param.page_size);
    let mut dept = None;
    match auth.user_level {
        // 获取所有帐号
        UserLevel::Admin => (),
        // 获取本部门帐号
        UserLevel::DeptAdmin => dept = Some(auth.dept_id),
        // 无权限
        UserLevel::Normal => return Err(APIError::new_permission_forbidden()),
    }

    let list = users::get_use_list(dept, status, (page - 1) * page_size, page_size).await?;
    let mut response = ApiResponse::ok_data(list);
    response.set_page(page, page_size);
    Ok(Json(response))
}

pub async fn register(ReqJson(param): ReqJson<RegisterParam>) -> APIResult<Json<ApiResponse<ID>>> {
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
    Ok(Json(ApiResponse::ok()))
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
) -> APIResult<(HeaderMap, Json<ApiResponse<AuthResponse>>)> {
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
    let auth = jwt::auth_token(user_info.id, user_info.dept_id, user_info.level);
    if auth.is_err() {
        tracing::error!("Token generation failure. err: {:?}", &auth);
        return Err(APIError::new_server_error());
    }
    let token = auth.unwrap();
    let header = jwt::set_cookie(&token);
    // 返回session
    Ok((header, Json(ApiResponse::ok_data(AuthResponse { token }))))
}
