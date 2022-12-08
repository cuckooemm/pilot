use crate::web::{
    api::{check, helper, permission::accredit},
    extract::{
        error::{APIError, ParamErrType},
        request::{ReqJson, ReqQuery},
        response::APIResponse,
    },
    store::dao::Dao,
    APIResult,
};

use axum::{extract::State, Extension};
use entity::{
    common::Id32Name,
    enums::Status,
    orm::{ActiveModelTrait, IntoActiveModel, Set},
    users::UserLevel,
    DepartmentModel, UserAuth,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DepartmentParam {
    pub id: Option<String>,
    pub name: Option<String>,
}

pub async fn create(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<DepartmentParam>,
) -> APIResult<APIResponse<DepartmentModel>> {
    let name = match param.name {
        Some(name) => {
            let name = check::trim(name);
            if name.len() == 0 || name.len() > 64 {
                return Err(APIError::param_err(ParamErrType::Len(1, 64), "name"));
            }
            name
        }
        None => return Err(APIError::param_err(ParamErrType::Required, "name")),
    };

    if !accredit::acc_admin(&auth, None).await? {
        return Err(APIError::forbidden_resource(
            crate::web::extract::error::ForbiddenType::Operate,
            &vec!["create", "department"],
        ));
    }

    if dao.department.is_exist(name.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "name"));
    }
    let data = dao.department.addition(name.clone()).await?;

    Ok(APIResponse::ok_data(data))
}

#[derive(Deserialize)]
pub struct EditDepartmentParam {
    pub id: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
}
// 更新部门信息
pub async fn edit(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<EditDepartmentParam>,
) -> APIResult<APIResponse<DepartmentModel>> {
    let id = check::id_decode::<u32>(param.id, "id")?;
    match auth.level {
        UserLevel::Admin => (),
        UserLevel::DeptAdmin => {
            // 不能修改其他部门的信息
            if auth.dept_id != id {
                return Err(APIError::forbidden_resource(
                    crate::web::extract::error::ForbiddenType::Operate,
                    &vec!["edit", "department"],
                ));
            }
        }
        _ => {
            return Err(APIError::forbidden_resource(
                crate::web::extract::error::ForbiddenType::Operate,
                &vec!["edit", "department"],
            ));
        }
    }
    let dept = dao
        .department
        .get_info(id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "id"))?;

    let mut active = dept.clone().into_active_model();
    if let Some(name) = param.name {
        let name = check::trim(name);
        if name.len() == 0 || name.len() > 64 {
            return Err(APIError::param_err(ParamErrType::Len(1, 64), "name"));
        }
        if name != dept.name {
            active.name = Set(name);
        }
    }

    if let Some(status) = param.status.and_then(|s| s.try_into().ok()) {
        if status != dept.status {
            active.status = Set(status);
        }
    }
    if !active.is_changed() {
        return Ok(APIResponse::ok_data(dept));
    }
    let model = dao.department.update(active).await?;
    Ok(APIResponse::ok_data(model))
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
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
) -> APIResult<APIResponse<Vec<Id32Name>>> {
    let name = match param.name {
        Some(name) => {
            let name = check::trim(name);
            if name.len() > 64 {
                return Err(APIError::param_err(ParamErrType::Len(1, 64), "name"));
            }
            if name.len() == 0 {
                None
            } else {
                Some(name)
            }
        }
        None => None,
    };
    let (page, page_size) = helper::page(param.page, param.page_size);
    let status: Option<Status> = param.status.and_then(|s| s.try_into().ok());

    let list = dao
        .department
        .search_department(name, status, helper::page_to_limit(page, page_size))
        .await?;
    Ok(APIResponse::ok_data(list))
}
