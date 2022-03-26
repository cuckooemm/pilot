use super::dao::cluster;
use super::response::{APIError, APIResponse, ParamErrType};
use super::APIResult;
use super::{check, ReqJson, ReqQuery};

use axum::extract::Json;
use entity::constant::APP_ID_MAX_LEN;
use entity::orm::Set;
use entity::{ClusterActive, ClusterModel, ID};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

const SECRET_LEN: usize = 36;

#[derive(Deserialize, Debug)]
pub struct ClusterParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
}

// 创建app集群
pub async fn create(ReqJson(param): ReqJson<ClusterParam>) -> APIResult<Json<APIResponse<ID>>> {
    // check param
    let cluster_name = check::name(param.cluster, "cluster")?;
    let app_id = check::appid_exist(param.app_id).await?;

    // 查看当前 app_id cluster_name是否存在
    let entity = cluster::is_exist(&app_id, &cluster_name).await?;
    if entity.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "cluster"));
    }
    let data = ClusterActive {
        app_id: Set(app_id),
        name: Set(cluster_name),
        secret: Set(general_rand_secret()),
        ..Default::default()
    };

    let id = cluster::insert(data).await?;
    Ok(Json(APIResponse::ok_data(ID::new(id))))
}

// 重置密钥接口
pub async fn reset_secret(
    ReqJson(param): ReqJson<ClusterParam>,
) -> APIResult<Json<APIResponse<ID>>> {
    let cluster_name = check::name(param.cluster, "cluster")?;
    let app_id = check::appid_exist(param.app_id).await?;
    let entity = cluster::find_by_cluster(app_id, cluster_name).await?;
    if entity.is_none() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "cluster"));
    }
    let entity = entity.unwrap();
    let mut active: ClusterActive = entity.clone().into();
    active.secret = Set(general_rand_secret());
    cluster::update_by_id(active, entity.id).await?;
    Ok(Json(APIResponse::ok()))
}

#[derive(Deserialize, Debug)]
pub struct ClusterQueryParam {
    pub app_id: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

pub async fn list(
    ReqQuery(param): ReqQuery<ClusterQueryParam>,
) -> APIResult<Json<APIResponse<Vec<ClusterModel>>>> {
    if let Some(app_id) = &param.app_id {
        if app_id.len() != 0 {
            if app_id.len() > APP_ID_MAX_LEN {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
        }
    }
    let (page, page_size) = check::page(param.page, param.page_size);
    let list: Vec<ClusterModel> =
        cluster::find_by_app_all(param.app_id, (page - 1) * page_size, page_size).await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page, page_size);
    Ok(Json(rsp))
}

fn general_rand_secret() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SECRET_LEN)
        .map(char::from)
        .collect()
}
