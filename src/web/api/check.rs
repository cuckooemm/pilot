use crate::web::extract::response::{APIError, ParamErrType};
use crate::web::store::dao::app;

use entity::orm::ConnectionTrait;
use entity::ID;
use once_cell::sync::Lazy;
use regex::Regex;

struct Re {
    id_str: Regex,
    account: Regex,
    password: Regex,
    email: Regex,
}

const ID_MIN_LEN: usize = 2;
const ID_MAX_LEN: usize = 80;

static RE: Lazy<Re> = Lazy::new(|| Re {
    id_str: Regex::new(r"^[a-z0-9_-]{6,80}$")
        .expect("Failed to initialize the [id_str] regular expression"),
    account: Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]{5,64}$")
        .expect("Failed to initialize the [account] regular expression"),
    password: Regex::new(r"[a-zA-Z0-9-*/+.~!@#$%^&*()]{6,64}$")
        .expect("Failed to initialize the [password] regular expression"),
    email: Regex::new(r"\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*")
        .expect("Failed to initialize the [email] regular expression"),
});

pub fn account(account: Option<String>) -> Result<String, APIError> {
    match account {
        Some(account) => {
            if account.len() < 6 || account.len() > 64 {
                return Err(APIError::new_param_err(ParamErrType::Len(6, 64), "account"));
            }
            if !(RE.account.is_match(&account) || RE.email.is_match(&account)) {
                return Err(APIError::new_param_err(ParamErrType::Invalid, "account"));
            }
            Ok(account)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "account")),
    }
}

pub fn password(password: Option<String>) -> Result<String, APIError> {
    match password {
        Some(password) => {
            if password.len() < 6 || password.len() > 64 {
                return Err(APIError::new_param_err(
                    ParamErrType::Len(6, 64),
                    "password",
                ));
            }
            if !RE.password.is_match(&password) {
                return Err(APIError::new_param_err(ParamErrType::Invalid, "password"));
            }
            Ok(password)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "password")),
    }
}

pub fn email(email: Option<String>) -> Result<String, APIError> {
    match email {
        Some(email) => {
            if email.len() < 6 || email.len() > 64 {
                return Err(APIError::new_param_err(ParamErrType::Len(6, 64), "email"));
            }
            if !RE.email.is_match(&email) {
                return Err(APIError::new_param_err(ParamErrType::Invalid, "email"));
            }
            Ok(email)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "email")),
    }
}

pub fn nickname(nickname: Option<String>) -> Result<Option<String>, APIError> {
    match nickname {
        Some(nickname) => {
            let nickname = nickname.trim().to_string();
            if nickname.len() < 2 || nickname.len() > 64 {
                return Err(APIError::new_param_err(
                    ParamErrType::Len(6, 64),
                    "nickname",
                ));
            }
            Ok(Some(nickname))
        }
        None => return Ok(None),
        // 如果没有传 nickname 使用 account 作为 nickname
    }
}

pub fn id_str(id: Option<String>, field: &str) -> Result<String, APIError> {
    match id {
        Some(id) => {
            if id.len() < ID_MIN_LEN || id.len() > ID_MAX_LEN {
                return Err(APIError::new_param_err(
                    ParamErrType::Len(ID_MIN_LEN, ID_MAX_LEN),
                    field,
                ));
            }
            if !RE.id_str.is_match(&id) {
                return Err(APIError::new_param_err(ParamErrType::Invalid, field));
            }
            Ok(id)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, field)),
    }
}

pub fn id_str_rule(id: String, field: &str) -> Result<String, APIError> {
    if id.len() < ID_MIN_LEN || id.len() > ID_MAX_LEN {
        return Err(APIError::new_param_err(
            ParamErrType::Len(ID_MIN_LEN, ID_MAX_LEN),
            field,
        ));
    }
    if !RE.id_str.is_match(&id) {
        return Err(APIError::new_param_err(ParamErrType::Invalid, field));
    }
    Ok(id)
}

pub fn id_decode(id: Option<String>, field: &str) -> Result<u64, APIError> {
    match id {
        Some(id) => {
            if id.len() == 0 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, field));
            }
            let id = entity::utils::decode_i64(&id);
            if id == 0 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, field));
            }
            Ok(id)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, field)),
    }
}

pub fn number(id: Option<i64>, field: &str) -> Result<i64, APIError> {
    match id {
        Some(id) => {
            if id == 0 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, field));
            }
            Ok(id)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, field)),
    }
}

pub fn page(page: Option<String>, page_size: Option<String>) -> (u64, u64) {
    let mut page = page.unwrap_or_default().parse::<u64>().unwrap_or(1);
    if page < 1 {
        page = 1;
    }

    let mut page_size = page_size.unwrap_or_default().parse::<u64>().unwrap_or(20);
    if page_size < 1 {
        page_size = 20;
    }
    if page_size > 10000 {
        page_size = 10000;
    }
    if u64::MAX / page_size < page {
        page = 1;
    }
    (page, page_size)
}

// 检查 appid 是否存在
pub async fn appid_exist(app_id: Option<String>) -> Result<String, APIError> {
    match app_id {
        Some(id) => {
            if id.len() < ID_MIN_LEN || id.len() > ID_MAX_LEN {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            // 查找 app_id 是否存在
            if !app::is_exist(id.clone()).await? {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            Ok(id)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "app_id")),
    }
}

pub async fn app_cluster_exist<'a, C>(
    db: &C,
    id: Option<String>,
    cluster: Option<String>,
) -> Result<String, APIError>
where
    C: ConnectionTrait,
{
    let id = match id {
        Some(id) => {
            if id.len() == 0 || id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            id
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "app_id")),
    };
    let cluster = match cluster {
        Some(cluster) => {
            if cluster.len() == 0 || cluster.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            cluster
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "cluster")),
    };

    Ok("".to_owned())
}
