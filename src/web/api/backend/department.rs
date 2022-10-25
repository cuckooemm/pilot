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
use chrono::Local;
use entity::{
    common::Id32Name,
    enums::Status,
    orm::{ActiveModelTrait, IntoActiveModel, Set},
    users::UserLevel,
    DepartmentModel, ID,
};
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

#[derive(Deserialize)]
pub struct EditDepartmentParam {
    pub id: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
}
// 更新部门信息
pub async fn edit(
    ReqJson(param): ReqJson<EditDepartmentParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<DepartmentModel>>> {
    let id = check::id_decode::<u32>(param.id, "id")?;
    match auth.user_level {
        UserLevel::Admin => (),
        UserLevel::DeptAdmin => {
            // 不能修改其他部门的信息
            if auth.dept_id != id {
                return Err(APIError::new_permission_forbidden());
            }
        }
        _ => return Err(APIError::new_permission_forbidden()),
    }
    let dept = department::get_info(id).await?;
    if dept.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    let dept = dept.unwrap();
    let mut active = dept.clone().into_active_model();
    if let Some(name) = param.name {
        let name = check::trim(name);
        if name.len() == 0 || name.len() > 128 {
            return Err(APIError::new_param_err(ParamErrType::Len(1, 128), "name"));
        }
        if name != dept.name {
            active.name = Set(name);
        }
    }
    if let Some(status) = param.status {
        match Status::from(status) {
            Status::Other => (),
            Status::Normal => {
                // 状态为delete 更改为正常
                if dept.deleted_at != 0 {
                    active.deleted_at = Set(0);
                }
            }
            Status::Delete => {
                if dept.deleted_at == 0 {
                    active.deleted_at = Set(Local::now().timestamp() as u64);
                }
            }
        }
    }
    if !active.is_changed() {
        return Ok(Json(ApiResponse::ok_data(dept)));
    }
    let model = department::update(active).await?;
    Ok(Json(ApiResponse::ok_data(model)))
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub name: Option<String>,
    pub status: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

pub async fn list(
    ReqQuery(param): ReqQuery<QueryParam>,
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
    let (page, page_size) = check::page(param.page, param.page_size);

    let status: Status = param.status.unwrap_or_default().into();
    let list =
        department::search_department(name, status, (page - 1) * page_size, page_size).await?;
    Ok(Json(ApiResponse::ok_data(list)))
}
