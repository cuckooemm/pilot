use super::error::APIError;

#[derive(Clone, Debug)]
pub enum ForbiddenType {
    Create,
    Edit,
    Access,
    Operate,
}

impl APIError {
    pub fn forbidden_resource(f_type: ForbiddenType, resource: &Vec<&str>) -> Self {
        match f_type {
            ForbiddenType::Access => Self {
                code: 403_000,
                message: format!("Forbidden access resource: [{}]", resource.join(".")),
            },
            ForbiddenType::Create => Self {
                code: 403_001,
                message: format!("Forbidden create resource: [{}]", resource.join(".")),
            },
            ForbiddenType::Edit => Self {
                code: 403_002,
                message: format!("Forbidden edit resource: [{}]", resource.join(".")),
            },
            ForbiddenType::Operate => Self {
                code: 403_003,
                message: format!("Forbidden operate resource: [{}]", resource.join(".")),
            },
        }
    }
}
