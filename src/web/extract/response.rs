use axum::{
    extract::rejection::JsonRejection,
    http::{header::HeaderName, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use entity::orm::DbErr;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Empty {}

#[derive(Debug, Serialize, Default)]
pub struct APIResponse<T: Serialize> {
    #[serde(rename(serialize = "code"))]
    pub code: i32,
    #[serde(rename(serialize = "message"))]
    pub message: String,
    #[serde(rename(serialize = "data"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(rename(serialize = "page"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename(serialize = "page_size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
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
    #[inline]
    pub fn new(code: i32, message: String, data: Option<T>) -> Self {
        Self {
            code,
            message,
            data,
            page: None,
            page_size: None,
        }
    }
    #[inline]
    pub fn ok_data(data: T) -> Self {
        Self::new(0, "OK".to_string(), Some(data))
    }
    #[inline]
    pub fn ok() -> Self {
        Self::new(0, "OK".to_string(), None)
    }
    #[inline]
    pub fn err(code: i32, message: String) -> Self {
        Self::new(code, message, None)
    }
    pub fn set_page(&mut self, page: u64, page_size: u64) {
        self.page = Some(page);
        self.page_size = Some(page_size);
    }
}

/// 错误的类型
pub enum APIErrorType {
    /// 未找到
    NotFound,
    /// 数据库错误
    Database,
    // 参数错误
    BadParam(ParamErrType),
    // 请求体错误
    BadRequestBody,
    // 服务内部异常
    ServerAbnormal,
    // 认证授权异常
    InvalidToken,
    // 无权限访问 拒绝访问
    Forbidden,
}

#[derive(Clone)]
pub enum ParamErrType {
    // 必填
    Required,
    // 长度
    Len(usize, usize),
    // 已存在
    Exist,
    // 不存在
    NotExist,
    // Invalid
    Invalid,
    // 已修改
    Changed,
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
        Self {
            error_type: APIErrorType::NotFound,
            message: None,
            cause: None,
        }
    }

    pub fn with_param(et: APIErrorType, msg: Option<String>) -> Self {
        Self {
            error_type: et,
            message: msg,
            cause: None,
        }
    }
    pub fn new_permission_forbidden() -> Self {
        Self {
            error_type: APIErrorType::Forbidden,
            message: None,
            cause: None,
        }
    }
    pub fn new_auth_invalid(msg: String) -> Self {
        Self {
            error_type: APIErrorType::InvalidToken,
            message: Some(msg),
            cause: None,
        }
    }
    pub fn new_server_error() -> Self {
        Self {
            error_type: APIErrorType::ServerAbnormal,
            message: None,
            cause: None,
        }
    }
    pub fn new_param_err(param_type: ParamErrType, field: &str) -> Self {
        Self {
            message: match param_type {
                ParamErrType::Required => Some(format!("The {} is required", field)),
                ParamErrType::Exist => Some(format!("The {} is exist", field)),
                ParamErrType::NotExist => Some(format!("The {} is not exist", field)),
                ParamErrType::Invalid => Some(format!("The {} is invalid", field)),
                ParamErrType::Changed => Some(format!("The {} is changed", field)),
                ParamErrType::Len(min, max) => Some(format!(
                    "The length of {} should be between {} and {}",
                    field, min, max
                )),
            },
            error_type: APIErrorType::BadParam(param_type),
            cause: None,
        }
    }
    pub fn new_db_err(db_err: DbErr) -> Self {
        tracing::error!("databases err: {:?}", db_err);
        let mut api_err = APIError::new();
        api_err.message = match db_err {
            DbErr::Exec(s) => {
                if s.contains("1062") && s.contains("Duplicate entry") {
                    api_err.error_type = APIErrorType::BadParam(ParamErrType::Exist);
                    Some("记录已存在".to_owned())
                } else {
                    api_err.error_type = APIErrorType::Database;
                    None
                }
            }
            DbErr::RecordNotFound(_s) => {
                api_err.error_type = APIErrorType::NotFound;
                None
            }
            DbErr::Custom(s) => {
                api_err.error_type = APIErrorType::BadParam(ParamErrType::Invalid);
                Some(s)
            }
            _ => {
                api_err.error_type = APIErrorType::Database;
                Some(format!("{}", db_err))
            }
        };
        api_err
    }
    pub fn new_parse_err(err: JsonRejection) -> Self {
        tracing::error!("parsing request body err: {}", err);
        let mut api_err = APIError::new();
        api_err.message = match err {
            JsonRejection::JsonDataError(err) => {
                api_err.error_type = APIErrorType::BadRequestBody;
                Some(err.to_string())
            }
            JsonRejection::JsonSyntaxError(err) => {
                api_err.error_type = APIErrorType::BadRequestBody;
                Some(err.to_string())
            }
            JsonRejection::MissingJsonContentType(err) => {
                api_err.error_type = APIErrorType::BadRequestBody;
                Some(err.to_string())
            }
            _ => {
                api_err.error_type = APIErrorType::BadRequestBody;
                Some(err.to_string())
            }
        };
        api_err
    }
}

impl From<DbErr> for APIError {
    fn from(err: DbErr) -> Self {
        APIError::new_db_err(err)
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let rsp: APIResponse<()> = match self.error_type {
            APIErrorType::BadParam(_) => {
                APIResponse::err(4000, self.message.unwrap_or("内部服务异常".to_owned()))
            }
            APIErrorType::BadRequestBody => {
                APIResponse::err(4000, self.message.unwrap_or("".to_owned()))
            }
            APIErrorType::InvalidToken => {
                APIResponse::err(4100, self.message.unwrap_or("认证无效".to_owned()))
            }
            APIErrorType::Forbidden => {
                APIResponse::err(4300, self.message.unwrap_or("无权限访问".to_owned()))
            }
            APIErrorType::ServerAbnormal => APIResponse::err(5000, "内部服务异常".to_owned()),
            APIErrorType::Database => APIResponse::err(5000, "内部服务异常".to_owned()),
            APIErrorType::NotFound => APIResponse::err(0, "OK".to_owned()),
        };
        let mut header = HeaderMap::new();
        header.insert(
            HeaderName::from_static("inner-status-code"),
            HeaderValue::from(rsp.code),
        );
        (StatusCode::OK, header, Json(rsp).into_response()).into_response()
    }
}
