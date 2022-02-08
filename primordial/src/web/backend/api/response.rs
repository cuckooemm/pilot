use axum::{
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct APIResponse<T: Serialize> {
    #[serde(rename(serialize = "code"))]
    pub code: i32,
    #[serde(rename(serialize = "message"))]
    pub message: String,
    #[serde(rename(serialize = "data"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

// impl<T> Display for APIResponse<T>
// where
//     T: Serialize,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "(code: {}, message: {}, data: {})",
//             self.code, self.message, self.data
//         )
//     }
// }

impl<T> APIResponse<T>
where
    T: Serialize,
{
    pub fn new(code: i32, message: String, data: Option<T>) -> Self {
        Self {
            code,
            message,
            data,
        }
    }
    pub fn ok(data: T) -> Self {
        Self::new(0, "OK".to_string(), Some(data))
    }
    pub fn err(code: i32, message: String) -> Self {
        Self::new(code, message, None)
    }
}

/// 错误的类型
pub enum APIErrorType {
    /// 未找到
    NotFound,
    /// 数据库错误
    Database,
}

/// API错误
pub struct APIError {
    /// 错误类型
    pub error_type: APIErrorType,
    /// 错误信息
    pub message: Option<String>,
    /// 错误原因（上一级的错误）
    pub cause: Option<String>,
}

impl APIError {
    pub fn new() -> Self {
        APIError {
            error_type: APIErrorType::NotFound,
            message: None,
            cause: None,
        }
    }
    pub fn new_db_err(db_err: DbErr) -> Self {
        let mut api_err = APIError::new();
        api_err.message = match db_err {
            DbErr::RecordNotFound(_s) => {
                api_err.error_type = APIErrorType::NotFound;
                None
            }
            _ => {
                api_err.error_type = APIErrorType::Database;
                Some(format!("{}", db_err))
            }
        };
        api_err
    }
}

impl From<sea_orm::DbErr> for APIError {
    fn from(err: sea_orm::DbErr) -> Self {
        APIError::new_db_err(err)
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let res: APIResponse<()> = match self.error_type {
            APIErrorType::Database => {
                tracing::error!("{}", self.message.unwrap_or("nothing".to_owned()));
                APIResponse::err(5000, "内部服务错误".to_owned())
            }
            APIErrorType::NotFound => APIResponse::err(0, "OK".to_owned()),
        };
        Json(res).into_response()
    }
}
