use crate::web::{
    api::{check, permission::accredit},
    extract::{
        json::ReqJson,
        jwt::Claims,
        query::ReqQuery,
        response::{APIError, ApiResponse, Empty, ParamErrType},
    },
    store::dao::department,
    APIResult,
};

use axum::Json;
use entity::{common::Id32Name, users::UserLevel, ID};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DepartmentParam {
    pub id: Option<String>,
    pub name: Option<String>,
}

pub async fn create(
    ReqJson(param): ReqJson<DepartmentParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<Id32Name>>> {
    let name = match param.name {
        Some(name) => {
            let name = check::trim(name);
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

    Ok(Json(ApiResponse::ok_data(Id32Name { id, name })))
}

// 更新部门信息
pub async fn edit(
    ReqJson(param): ReqJson<DepartmentParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<Id32Name>>> {
    let id = check::id_decode::<u32>(param.id, "id")?;
    let dept_name = department::get_department_name(id).await?;
    if dept_name.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    let mut dept_name = dept_name.unwrap();
    match auth.user_level {
        UserLevel::Admin => (),
        UserLevel::DeptAdmin => {
            // 不能修改其他部门的信息
            if auth.dept_id != id {
                return Err(APIError::new_permission_forbidden());
            }
        }
        UserLevel::Normal => return Err(APIError::new_permission_forbidden()),
    }
    if let Some(name) = param.name {
        let name = check::trim(name);
        if name.len() == 0 || name.len() > 128 {
            return Err(APIError::new_param_err(ParamErrType::Len(1, 128), "name"));
        }
        if name != dept_name {
            department::update(id, name.clone()).await?;
            dept_name = name;
        }
    }
    Ok(Json(ApiResponse::ok_data(Id32Name {
        id,
        name: dept_name,
    })))
}

pub async fn list(
    ReqQuery(param): ReqQuery<DepartmentParam>,
    _: Claims,
) -> APIResult<Json<ApiResponse<Vec<Id32Name>>>> {
    let name = match param.name {
        Some(name) => {
            let name = check::trim(name);
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
    Ok(Json(ApiResponse::ok_data(list)))
}

pub async fn delete(
    ReqJson(param): ReqJson<DepartmentParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<Empty>>> {
    let name = match param.name {
        Some(name) => {
            let name = check::trim(name);
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
    Ok(Json(ApiResponse::ok()))
}
