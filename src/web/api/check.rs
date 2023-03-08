use crate::web::extract::error::{APIError, ParamErrType};

use once_cell::sync::Lazy;
use regex::Regex;

struct Re {
    id_str: Regex,
    account: Regex,
    password: Regex,
    email: Regex,
    key: Regex,
}

static RE: Lazy<Re> = Lazy::new(|| Re {
    id_str: Regex::new(r"^[a-z0-9_-]*$")
        .expect("Failed to initialize the [id_str] regular expression"),
    account: Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$")
        .expect("Failed to initialize the [account] regular expression"),
    password: Regex::new(r"[a-zA-Z0-9-*/+.~!@#$%^&*()]$")
        .expect("Failed to initialize the [password] regular expression"),
    email: Regex::new(r"\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*")
        .expect("Failed to initialize the [email] regular expression"),
    key: Regex::new(r"^[a-zA-Z\d_-]*$").expect("Failed to initialize the [key] regular expression"),
});

pub fn account(account: Option<String>) -> Result<String, APIError> {
    match account {
        Some(account) => {
            account_rule(&account)?;
            Ok(account)
        }
        None => return Err(APIError::param_err(ParamErrType::Required, "account")),
    }
}
pub fn account_rule(account: &String) -> Result<(), APIError> {
    if account.len() < 5 || account.len() > 64 {
        return Err(APIError::param_err(ParamErrType::Len(5, 64), "account"));
    }
    if !(RE.account.is_match(&account) || RE.email.is_match(&account)) {
        return Err(APIError::param_err(ParamErrType::Invalid, "account"));
    }
    Ok(())
}

pub fn password(password: Option<String>) -> Result<String, APIError> {
    match password {
        Some(password) => {
            password_rule(&password)?;
            Ok(password)
        }
        None => return Err(APIError::param_err(ParamErrType::Required, "password")),
    }
}
pub fn password_rule(password: &String) -> Result<(), APIError> {
    if password.len() < 6 || password.len() > 64 {
        return Err(APIError::param_err(ParamErrType::Len(6, 64), "password"));
    }
    if !RE.password.is_match(&password) {
        return Err(APIError::param_err(ParamErrType::Invalid, "password"));
    }
    Ok(())
}

pub fn email(email: Option<String>) -> Result<String, APIError> {
    match email {
        Some(email) => {
            email_rule(&email)?;
            Ok(email)
        }
        None => return Err(APIError::param_err(ParamErrType::Required, "email")),
    }
}
pub fn email_rule(email: &String) -> Result<(), APIError> {
    if email.len() < 6 || email.len() > 64 {
        return Err(APIError::param_err(ParamErrType::Len(6, 64), "email"));
    }
    if !RE.email.is_match(&email) {
        return Err(APIError::param_err(ParamErrType::Invalid, "email"));
    }
    Ok(())
}

pub fn nickname(nickname: Option<String>) -> Result<Option<String>, APIError> {
    match nickname {
        Some(nickname) => {
            let nickname = nickname_rule(nickname)?;
            Ok(Some(nickname))
        }
        None => return Ok(None),
        // 如果没有传 nickname 使用 account 作为 nickname
    }
}
pub fn nickname_rule(nickname: String) -> Result<String, APIError> {
    let nickname = trim(nickname);
    if nickname.len() < 2 || nickname.len() > 26 {
        return Err(APIError::param_err(ParamErrType::Len(2, 26), "nickname"));
    }
    Ok(nickname)
}

pub fn trim(f: String) -> String {
    f.trim_start().trim_end().to_owned()
}

pub fn key(id: Option<String>, field: &str) -> Result<String, APIError> {
    match id {
        Some(id) => {
            key_rule(&id, field)?;
            Ok(id)
        }
        None => return Err(APIError::param_err(ParamErrType::Required, field)),
    }
}

pub fn key_rule(id: &String, field: &str) -> Result<(), APIError> {
    if id.len() < 2 || id.len() > 64 {
        return Err(APIError::param_err(ParamErrType::Len(2, 64), field));
    }
    if !RE.key.is_match(id) {
        return Err(APIError::param_err(ParamErrType::Invalid, field));
    }
    Ok(())
}

pub fn id_str(id: Option<String>, field: &str) -> Result<String, APIError> {
    match id {
        Some(id) => Ok(id_str_rule(id, field)?),
        None => return Err(APIError::param_err(ParamErrType::Required, field)),
    }
}

pub fn id_str_rule(id: String, field: &str) -> Result<String, APIError> {
    if id.len() < 2 || id.len() > 64 {
        return Err(APIError::param_err(
            ParamErrType::Len(2, 64),
            field,
        ));
    }
    if !RE.id_str.is_match(&id) {
        return Err(APIError::param_err(ParamErrType::Invalid, field));
    }
    Ok(id)
}

pub fn id_decode<T: TryFrom<u64>>(id: Option<String>, field: &str) -> Result<T, APIError> {
    match id {
        Some(id) => id_decode_rule::<T>(&id, field),
        None => return Err(APIError::param_err(ParamErrType::Required, field)),
    }
}

pub fn id_decode_rule<T: TryFrom<u64>>(id: &String, field: &str) -> Result<T, APIError> {
    if id.len() == 0 {
        return Err(APIError::param_err(ParamErrType::NotExist, field));
    }
    let id = entity::utils::decode_u64(id);
    if id == 0 {
        return Err(APIError::param_err(ParamErrType::NotExist, field));
    }
    T::try_from(id).map_err(|_| APIError::param_err(ParamErrType::NotExist, field))
}
