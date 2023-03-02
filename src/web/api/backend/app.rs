use crate::web::api::permission::accredit;
use crate::web::api::{check, helper};
use crate::web::extract::error::{APIError, ParamErrType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::Dao;
use crate::web::APIResult;

use axum::extract::State;
use axum::Extension;
use entity::model::{rule::Verb, AppActive, AppModel, UserAuth};
use entity::orm::{ActiveModelTrait, IntoActiveModel, Set};
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct AppParam {
    pub app: Option<String>,
    pub name: Option<String>,
    pub describe: Option<String>,
    pub dept_id: Option<String>,
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
    let dept_id = if let Some(id) = param.dept_id {
        let id = check::id_decode_rule::<u32>(&id, "dept_id")?;
        if !dao.department.is_exist_id(id).await? {
            return Err(APIError::param_err(ParamErrType::NotExist, "dept_id"));
        }
        id
    } else {
        auth.dept_id
    };
    if dept_id != auth.dept_id {
        todo!("check create app premission")
    }
    if dao.app.is_exist(app.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "app"));
    }
    let active = AppActive {
        app: Set(app.clone()),
        name: Set(name),
        dept_id: Set(dept_id),
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
    pub dept_id: Option<String>,
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
        return Err(APIError::forbidden_resource(
            crate::web::extract::error::ForbiddenType::Operate,
            &resource,
        ));
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
    if let Some(dept_id) = param.dept_id {
        let dept_id = check::id_decode_rule::<u32>(&dept_id, "dept_id")?;
        if dept_id != info.dept_id {
            if !dao.department.is_exist_id(dept_id).await? {
                return Err(APIError::param_err(ParamErrType::NotExist, "dept_id"));
            }
            active.dept_id = Set(dept_id);
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
    // accredit::accredit(&auth, Verb::VIEW, vec!["some_app"]).await?;
    // TODO user app
    let (page, page_size) = helper::page(param.page, param.page_size);
    let list = dao
        .app
        .find_all(helper::page_to_limit(page, page_size))
        .await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page, page_size);
    Ok(rsp)
}
