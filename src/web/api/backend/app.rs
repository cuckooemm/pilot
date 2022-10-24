use crate::web::api::check;
use crate::web::api::permission::accredit;
use crate::web::extract::json::ReqJson;
use crate::web::extract::jwt::Claims;
use crate::web::extract::query::ReqQuery;
use crate::web::extract::response::{APIError, ApiResponse, Empty, ParamErrType};
use crate::web::store::dao::{app, department};
use crate::web::APIResult;

use axum::extract::Json;
use chrono::Local;
use entity::enums::Status;
use entity::orm::{ActiveModelTrait, IntoActiveModel, Set};
use entity::AppModel;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppParam {
    pub app_id: Option<String>,
    pub name: Option<String>,
    pub dept_id: Option<String>,
}

// 创建APP
pub async fn create(
    ReqJson(param): ReqJson<AppParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<Empty>>> {
    let app_id = check::id_str(param.app_id, "app_id")?;
    let name = match param.name {
        Some(name) => {
            let name = check::trim(name);
            if name.len() < 2 || name.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::Len(2, 100), "name"));
            }
            name
        }
        None => app_id.clone(),
    };
    let dept_id = check::id_decode::<u32>(param.dept_id, "dept_id")?;
    if !department::is_exist_id(dept_id).await? {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "dept_id"));
    }
    if app::is_exist(app_id.clone()).await? {
        return Err(APIError::new_param_err(ParamErrType::Exist, "app_id"));
    }
    app::add(app_id, name, dept_id, &auth).await?;
    Ok(Json(ApiResponse::ok()))
}

#[derive(Deserialize)]
pub struct EditParam {
    pub app_id: Option<String>,
    pub name: Option<String>,
    pub dept_id: Option<String>,
    pub status: Option<String>,
}

pub async fn edit(
    ReqJson(param): ReqJson<EditParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<AppModel>>> {
    let app_id = check::id_str(param.app_id, "app_id")?;
    let info = app::get_info(app_id.clone()).await?;
    if info.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
    }
    if !accredit::accredit(&auth, entity::rule::Verb::Modify, vec![&app_id]).await? {
        return Err(APIError::new_permission_forbidden());
    }
    let info = info.unwrap();
    let mut active = info.clone().into_active_model();
    if let Some(name) = param.name {
        let name = check::trim(name);
        if name.len() < 2 || name.len() > 100 {
            return Err(APIError::new_param_err(ParamErrType::Len(2, 100), "name"));
        }
        if name != info.name {
            active.name = Set(name);
        }
    }
    if let Some(dept_id) = param.dept_id {
        let dept_id = check::id_decode_rule::<u32>(&dept_id, "dept_id")?;
        if dept_id != info.dept_id {
            if !department::is_exist_id(dept_id).await? {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "dept_id"));
            }
            active.dept_id = Set(dept_id);
        }
    }

    if let Some(status) = param.status {
        match Status::from(status) {
            Status::Other => (),
            Status::Normal => {
                // 状态为delete 更改为正常
                if info.deleted_at != 0 {
                    active.deleted_at = Set(0);
                }
            }
            Status::Delete => {
                if info.deleted_at == 0 {
                    active.deleted_at = Set(Local::now().timestamp() as u64);
                }
            }
        }
    };

    if !active.is_changed() {
        return Ok(Json(ApiResponse::ok_data(info)));
    }
    let model = app::update(active).await?;
    Ok(Json(ApiResponse::ok_data(model)))
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub page: Option<String>,
    pub page_size: Option<String>,
}

// 获取用户APP
pub async fn list(
    ReqQuery(param): ReqQuery<QueryParam>,
    _: Claims,
) -> APIResult<Json<ApiResponse<Vec<AppModel>>>> {
    // accredit::accredit(&auth, Verb::VIEW, vec!["some_app"]).await?;
    let (page, page_size) = check::page(param.page, param.page_size);
    let list = app::find_all((page - 1) * page_size, page_size).await?;
    let mut rsp = ApiResponse::ok_data(list);
    rsp.set_page(page, page_size);
    Ok(Json(rsp))
}
