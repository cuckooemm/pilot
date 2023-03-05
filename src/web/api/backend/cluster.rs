use super::APIResult;
use crate::web::api::permission::accredit;
use crate::web::api::{check, helper};
use crate::web::extract::error::{APIError, ForbiddenType, ParamErrType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::Dao;

use axum::extract::State;
use axum::Extension;
use entity::common::enums::Status;
use entity::model::{rule::Verb, ClusterActive, ClusterModel, UserAuth};
use entity::orm::{ActiveModelTrait, IntoActiveModel, Set};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use tracing::instrument;

const SECRET_LEN: usize = 36;

#[derive(Deserialize, Debug)]
pub struct ClusterParam {
    pub app: Option<String>,
    pub cluster: Option<String>,
    pub describe: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn create(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<ClusterParam>,
) -> APIResult<APIResponse<ClusterModel>> {
    let app = check::id_str(param.app, "app")?;
    let cluster = check::id_str(param.cluster, "cluster")?;
    let describe = if let Some(desc) = param.describe {
        if desc.len() > 200 {
            return Err(APIError::param_err(ParamErrType::Max(200), "describe"));
        }
        desc
    } else {
        String::default()
    };
    if !dao.app.is_exist(app.clone()).await? {
        return Err(APIError::param_err(ParamErrType::NotExist, "app"));
    }
    let resouce = vec![app.as_str()];
    if !accredit::accredit(&auth, Verb::Create, &resouce).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Create,
            &resouce,
        ));
    }
    if dao.cluster.is_exist(app.clone(), cluster.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "cluster"));
    }

    let active = ClusterActive {
        app: Set(app),
        cluster: Set(cluster),
        secret: Set(general_rand_secret()),
        describe: Set(describe),
        ..Default::default()
    };
    let data = dao.cluster.addition(active).await?;
    Ok(APIResponse::ok_data(data))
}

#[derive(Deserialize, Debug)]
pub struct EditParam {
    pub id: Option<String>,
    pub reset_secret: Option<bool>,
    pub descibe: Option<String>,
    pub status: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn edit(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<EditParam>,
) -> APIResult<APIResponse<ClusterModel>> {
    let id = check::id_decode::<u64>(param.id, "id")?;
    let cluster = dao
        .cluster
        .get_cluster_by_id(id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "id"))?;
    let resource = vec![cluster.app.as_str(), cluster.cluster.as_str()];
    if !accredit::accredit(&auth, Verb::Modify, &resource).await? {
        return Err(APIError::forbidden_resource(ForbiddenType::Edit, &resource));
    }
    let mut active = cluster.clone().into_active_model();
    if let Some(desc) = param.descibe {
        if desc != cluster.describe {
            if desc.len() > 200 {
                return Err(APIError::param_err(ParamErrType::Max(200), "describe"));
            }
            active.describe = Set(desc);
        }
    }
    if let Some(status) = param.status.and_then(|s| s.try_into().ok()) {
        if status != cluster.status {
            active.status = Set(status);
        }
    }
    if param.reset_secret.unwrap_or_default() {
        active.secret = Set(general_rand_secret());
    }
    if !active.is_changed() {
        return Ok(APIResponse::ok_data(cluster));
    }
    let data = dao.cluster.update(active).await?;
    Ok(APIResponse::ok_data(data))
}

#[derive(Deserialize, Debug)]
pub struct ClusterQueryParam {
    pub app: Option<String>,
    pub status: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn list(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqQuery(param): ReqQuery<ClusterQueryParam>,
) -> APIResult<APIResponse<Vec<ClusterModel>>> {
    let app = check::id_str(param.app, "app")?;
    let status: Option<Status> = param.status.and_then(|s| s.try_into().ok());
    let page = helper::page(param.page, param.page_size);
    let resource = vec![app.as_str()];
    if accredit::accredit(&auth, Verb::VIEW, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Access,
            &resource,
        ));
    }
    let list = dao
        .cluster
        .find_cluster_by_app(app.clone(), status, page)
        .await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page);
    Ok(rsp)
}

fn general_rand_secret() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SECRET_LEN)
        .map(char::from)
        .collect()
}
