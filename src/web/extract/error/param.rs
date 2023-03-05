use super::error::APIError;

#[derive(Clone, Debug)]
pub enum ParamErrType {
    Required,
    Len(usize, usize),
    Min(i32),
    Max(i32),
    Exist,
    NotExist,
    Invalid,
    Changed,
}

impl APIError {
    pub fn param_err(param_type: ParamErrType, field: &str) -> Self {
        let (code, message) = match param_type {
            ParamErrType::Required => (400_001, format!("The {} is required", field)),
            ParamErrType::Exist => (400_002, format!("The {} is exist", field)),
            ParamErrType::Invalid => (400_003, format!("The {} is invalid", field)),
            ParamErrType::NotExist => (400_004, format!("The {} is not exist", field)),
            ParamErrType::Changed => (400_005, format!("The {} is changed", field)),
            ParamErrType::Min(length) => (
                400_006,
                format!("The minimum length of the {} is {}", field, length),
            ),
            ParamErrType::Max(length) => (
                400_007,
                format!("The maximum length of the {} is {}", field, length),
            ),
            ParamErrType::Len(min, max) => (
                400_008,
                format!(
                    "The length of {} should be between {} and {}",
                    field, min, max
                ),
            ),
        };
        Self { code, message }
    }
}
