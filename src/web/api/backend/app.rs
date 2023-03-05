use crate::web::api::permission::accredit::{self, acc_admin};
use crate::web::api::{check, helper};
use crate::web::extract::error::{APIError, ForbiddenType, ParamErrType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::Dao;
use crate::web::APIResult;

use axum::extract::State;
use axum::Extension;
use entity::common::enums::Status;
use entity::model::users::UserLevel;
use entity::model::{rule::Verb, AppActive, AppModel, UserAuth};
use entity::orm::{ActiveModelTrait, IntoActiveModel, Set};
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct AppParam {
    pub app: Option<String>,
    pub name: Option<String>,
    pub describe: Option<String>,
    pub department_id: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn create(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<AppParam>,
) -> APIResult<APIResponse<AppModel>> {
    let app = check::id_str(param.app, "app")?;
    let name = match param.name {
        Some(name) => {
            let name = check::trim(name);
            if name.len() < 2 || name.len() > 64 {
                return Err(APIError::param_err(ParamErrType::Len(2, 64), "name"));
            }
            name
        }
        None => app.clone(),
    };
    let describe = match param.describe {
        Some(desc) => {
            if desc.len() > 200 {
                return Err(APIError::param_err(ParamErrType::Max(200), "describe"));
            }
            desc
        }
        None => String::default(),
    };

    let dept_id = if let Some(id) = param.department_id {
        let id = check::id_decode_rule::<u32>(&id, "department_id")?;
        if !dao.department.is_exist_id(id).await? {
            return Err(APIError::param_err(ParamErrType::NotExist, "department_id"));
        }
        match auth.level {
            UserLevel::Admin => (),
            _ => {
                if id != auth.id {
                    return Err(APIError::forbidden_resource(
                        ForbiddenType::Create,
                        &vec!["department", "app"],
                    ));
                }
            }
        }
        id
    } else {
        auth.dept_id
    };
    if dao.app.is_exist(app.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "app"));
    }
    let active = AppActive {
        app: Set(app),
        name: Set(name),
        department_id: Set(dept_id),
        describe: Set(describe),
        ..Default::default()
    };
    let data = dao.app.addition(active, auth.id).await?;
    Ok(APIResponse::ok_data(data))
}

#[derive(Deserialize, Debug)]
pub struct EditParam {
    pub app: Option<String>,
    pub name: Option<String>,
    pub department_id: Option<String>,
    pub describe: Option<String>,
    pub status: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn edit(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<EditParam>,
) -> APIResult<APIResponse<AppModel>> {
    let app = check::id_str(param.app, "app")?;
    let info = dao
        .app
        .get_info(app.clone())
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "app"))?;
    let resource = vec![app.as_str()];
    if !accredit::accredit(&auth, Verb::Modify, &resource).await? {
        return Err(APIError::forbidden_resource(ForbiddenType::Edit, &resource));
    }

    let mut active = info.clone().into_active_model();
    if let Some(name) = param.name {
        let name = check::trim(name);
        if name.len() < 2 || name.len() > 64 {
            return Err(APIError::param_err(ParamErrType::Len(2, 64), "name"));
        }
        if name != info.name {
            active.name = Set(name);
        }
    }
    if let Some(desc) = param.describe {
        if desc.len() > 200 {
            return Err(APIError::param_err(ParamErrType::Max(200), "describe"));
        }
        if desc != info.describe {
            active.describe = Set(desc);
        }
    }
    if let Some(dept_id) = param.department_id {
        let dept_id = check::id_decode_rule::<u32>(&dept_id, "dept_id")?;
        if dept_id != info.department_id {
            if !acc_admin(&auth, None).await? {
                return Err(APIError::forbidden_resource(ForbiddenType::Edit, &resource));
            }
            if !dao.department.is_exist_id(dept_id).await? {
                return Err(APIError::param_err(ParamErrType::NotExist, "dept_id"));
            }
            active.department_id = Set(dept_id);
        }
    }

    if let Some(status) = param.status.and_then(|s| s.try_into().ok()) {
        if status != info.status {
            active.status = Set(status);
        }
    };

    if !active.is_changed() {
        return Ok(APIResponse::ok_data(info));
    }
    let data = dao.app.update(active).await?;
    Ok(APIResponse::ok_data(data))
}

#[derive(Deserialize, Debug)]
pub struct QueryParam {
    pub status: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn list(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqQuery(param): ReqQuery<QueryParam>,
) -> APIResult<APIResponse<Vec<AppModel>>> {
    let page = helper::page(param.page, param.page_size);
    let status: Option<Status> = param.status.and_then(|s| s.try_into().ok());
    let list = dao
        .app
        .get_apps(Some(auth.dept_id), status, helper::page_to_limit(page))
        .await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page);
    Ok(rsp)
}
