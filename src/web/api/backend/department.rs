use crate::web::{
    api::permission::accredit,
    extract::{
        json::ReqJson,
        jwt::Claims,
        query::ReqQuery,
        response::{APIError, APIResponse, ParamErrType},
    },
    store::dao::department,
    APIResult,
};

use axum::Json;
use entity::{common::Id32Name, ID};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DepartmentParam {
    pub name: Option<String>,
}

pub async fn create(
    ReqJson(param): ReqJson<DepartmentParam>,
    auth: Claims,
) -> APIResult<Json<APIResponse<Id32Name>>> {
    let name = match param.name {
        Some(name) => {
            let name = name.trim_start().trim_end().to_owned();
            if name.len() == 0 || name.len() > 128 {
                return Err(APIError::new_param_err(ParamErrType::Len(1, 128), "name"));
            }
            name
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "name")),
    };

    // 仅超级管理员可操作
    if !accredit::acc_admin(&auth, None) {
        return Err(APIError::new_permission_forbidden());
    }

    // 查看部门名是否存在
    if department::is_exist(name.clone()).await? {
        return Err(APIError::new_param_err(ParamErrType::Exist, "name"));
    }
    let id = department::add(name.clone()).await?;

    Ok(Json(APIResponse::ok_data(Id32Name { id, name })))
}

pub async fn list(
    ReqQuery(param): ReqQuery<DepartmentParam>,
    _: Claims,
) -> APIResult<Json<APIResponse<Vec<Id32Name>>>> {
    let name = match param.name {
        Some(name) => {
            let name = name.trim_start().trim_end().to_owned();
            if name.len() > 128 {
                return Err(APIError::new_param_err(ParamErrType::Len(1, 128), "name"));
            }
            if name.len() == 0 {
                None
            } else {
                Some(name)
            }
        }
        None => None,
    };
    let list = department::search_department(name).await?;
    Ok(Json(APIResponse::ok_data(list)))
}

pub async fn delete(
    ReqJson(param): ReqJson<DepartmentParam>,
    auth: Claims,
) -> APIResult<Json<APIResponse<ID>>> {
    let name = match param.name {
        Some(name) => {
            let name = name.trim_start().trim_end().to_owned();
            if name.len() == 0 || name.len() > 128 {
                return Err(APIError::new_param_err(ParamErrType::Len(1, 128), "name"));
            }
            name
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "name")),
    };

    // 仅超级管理员可操作
    if !accredit::acc_admin(&auth, None) {
        return Err(APIError::new_permission_forbidden());
    }
    let row = department::delete(name).await?;
    if row == 0 {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "name"));
    }
    Ok(Json(APIResponse::ok()))
}
