use super::dao::{cluster, namespace};
use super::ConfigList;
use crate::web::{
    extract::{
        query::ReqQuery,
        response::{APIError, APIResponse, ParamErrType},
    },
    APIResult,
};

use axum::Json;
use entity::constant::NAME_MAX_LEN;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DescParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
    pub namespace: Option<String>,
    pub secret: Option<String>,
}

// 全量获取配置数据
pub async fn description(
    ReqQuery(param): ReqQuery<DescParam>,
) -> APIResult<Json<APIResponse<ConfigList>>> {
    let app_id = match param.app_id {
        Some(app_id) => {
            if app_id.len() == 0 || app_id.len() > NAME_MAX_LEN {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            app_id
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "app_id")),
    };

    let cluster = match param.cluster {
        Some(cluster) => {
            if cluster.len() == 0 || cluster.len() > NAME_MAX_LEN {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "cluster"));
            }
            cluster
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "cluster")),
    };
    let namespace = match &param.namespace {
        Some(ns) => {
            if ns.len() == 0 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
            }
            ns.split(",").collect::<Vec<_>>()
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "namespace")),
    };

    // 查看 cluster 是否存在 且获取到 secret
    let secret = cluster::get_secret_by_cluster(&app_id, &cluster).await?;
    if secret.is_none() {
        return Ok(Json(APIResponse::ok(None)));
    }
    // 根据namespace获取key
    let ns_list = namespace::find_namespaceid_by_app_cluster(&app_id, &cluster, &namespace).await?;
    if ns_list.is_empty() {
        return Ok(Json(APIResponse::ok(None)));
    }
    // 获取当前ID key
    let mut ns_ids = Vec::with_capacity(ns_list.len());
    for id in ns_list.iter() {
        ns_ids.push(id.id);
    }
    // 获取当前 namespace key

    Ok(Json(APIResponse::ok(None)))
}

// 阻塞链接, 仅更新时返回数据
pub async fn notifaction(ReqQuery(param): ReqQuery<DescParam>) -> String {
    // let result = entity::AppEntity::find().all(&store.db).await;
    // tracing::info!("receive param {:?},result {:?}", &param, &result);
    format!(
        "receive param {:?} {:?} {:?}",
        param.app_id, param.namespace, param.secret
    )
}
