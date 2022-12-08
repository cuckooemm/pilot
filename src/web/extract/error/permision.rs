use super::error::APIError;

#[derive(Clone, Debug)]
pub enum ForbiddenType {
    Access,
    Operate,
}

impl APIError {
    pub fn forbidden_err(f_type: ForbiddenType, resource: &str) -> Self {
        match f_type {
            ForbiddenType::Access => Self {
                code: 403_000,
                message: format!("Forbidden access resource: [{}]", resource),
            },
            ForbiddenType::Operate => Self {
                code: 403_001,
                message: format!("Forbidden operate resource: [{}]", resource),
            },
        }
    }
    pub fn forbidden_resource(f_type: ForbiddenType, resource: &Vec<&str>) -> Self {
        match f_type {
            ForbiddenType::Access => Self {
                code: 403_000,
                message: format!("Forbidden access resource: [{}]", resource.join(".")),
            },
            ForbiddenType::Operate => Self {
                code: 403_001,
                message: format!("Forbidden operate resource: [{}]", resource.join(".")),
            },
        }
    }
}
