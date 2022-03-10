use super::orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use super::response::{APIError, APIResponse, ParamErrType};
use super::{check, ReqJson};
use super::{APIResult, ClusterActive, ClusterColumn, ClusterEntity, ClusterModel, ID};

use axum::extract::{Extension, Json, Query};
use entity::dao::cluster;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ClusterParam {
    pub app_id: Option<String>,
    pub name: Option<String>,
}

// 创建app集群
pub async fn create(
    ReqJson(param): ReqJson<ClusterParam>,
) -> APIResult<Json<APIResponse<ClusterModel>>> {
    // check param
    let cluster_name = check::name(param.name, "name")?;
    let app_id = check::appid_exist(param.app_id).await?;

    // 查看当前 app_id cluster_name是否存在
    let entity = cluster::is_exist(&app_id, &cluster_name).await?;
    if entity.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "cluster_name"));
    }
    let data = ClusterActive {
        app_id: Set(app_id),
        name: Set(cluster_name),
        // secret TODO
        ..Default::default()
    };

    let result = cluster::insert_one(data).await?;
    Ok(Json(APIResponse::ok(Some(result))))
}

pub async fn list(
    Query(param): Query<ClusterParam>,
) -> APIResult<Json<APIResponse<Vec<ClusterModel>>>> {
    if let Some(app_id) = &param.app_id {
        if app_id.len() != 0 {
            if app_id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
        }
    }
    let list: Vec<ClusterModel> = cluster::find_by_app_all(param.app_id).await?;

    Ok(Json(APIResponse::ok(Some(list))))
}
