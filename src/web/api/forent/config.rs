use std::time::Duration;

use super::dao::{cluster, namespace};
use super::ConfigList;
use crate::web::store::cache::{CacheItem, NamespaceItem};
use crate::web::{
    extract::{
        query::ReqQuery,
        response::{APIError, APIResponse, ParamErrType},
    },
    APIResult,
};

use axum::extract::Extension;
use axum::Json;
use entity::constant::{APP_ID_MAX_LEN, NAME_MAX_LEN};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tokio::time;

#[derive(Serialize, Deserialize, Debug)]
pub struct DescParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
    pub namespace: Option<String>,
    pub secret: Option<String>,
    pub version: Option<String>,
    pub timeout: Option<u64>,
}

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

// 全量获取配置数据
pub async fn description(
    ReqQuery(param): ReqQuery<DescParam>,
    Extension(cache): Extension<CacheItem>,
) -> APIResult<Json<APIResponse<Vec<NamespaceItem>>>> {
    let app_id = match param.app_id {
        Some(app_id) => {
            if app_id.len() == 0 || app_id.len() > APP_ID_MAX_LEN {
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
    let timeout = match param.timeout {
        Some(tm) => {
            if tm < 10 || tm > DEFAULT_TIMEOUT.as_secs_f64() as u64 {
                DEFAULT_TIMEOUT
            } else {
                Duration::from_secs(tm)
            }
        }
        None => DEFAULT_TIMEOUT,
    };
    // 查看 cluster 是否存在 且获取到 secret
    let secret = cluster::get_secret_by_cluster(&app_id, &cluster).await?;
    if secret.is_none() {
        return Ok(Json(APIResponse::ok()));
    }
    // TODO 校验secret
    // 根据namespace获取key
    let ns_list = namespace::find_namespaceid_by_app_cluster(&app_id, &cluster, &namespace).await?;
    if ns_list.is_empty() {
        return Ok(Json(APIResponse::ok()));
    }
    // 获取当前ID key
    let mut ns_ids = Vec::with_capacity(ns_list.len());
    for id in ns_list.iter() {
        ns_ids.push(id.id);
    }
    // 获取当前 namespace key
    // let (cancen_sender,cancen_receiver) = oneshot::channel()

    let result = cache.subscription(1, None).await;
    if result.is_none() {
        return Ok(Json(APIResponse::ok()));
    }
    let rsp = vec![result.unwrap()];
    Ok(Json(APIResponse::ok_data(rsp)))
}

// 阻塞链接, 仅更新时返回数据
pub async fn notifaction(
    ReqQuery(param): ReqQuery<DescParam>,
    Extension(cache): Extension<CacheItem>,
) -> APIResult<Json<APIResponse<Vec<NamespaceItem>>>> {
    // let result = entity::AppEntity::find().all(&store.db).await;
    // tracing::info!("receive param {:?},result {:?}", &param, &result);
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
    let version = match param.version {
        Some(version) => {
            if version.len() != 32 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "version"));
            }
            version
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "version")),
    };
    let timeout = match param.timeout {
        Some(tm) => {
            // 如果设置的超时时间过长或过短 则使用默认超时时间
            if tm < 10 || tm > DEFAULT_TIMEOUT.as_secs_f64() as u64 {
                DEFAULT_TIMEOUT
            } else {
                Duration::from_secs(tm) // 冗余传输时间
            }
        }
        None => DEFAULT_TIMEOUT,
    };

    let namespace_item = time::timeout(timeout, async {
        cache.subscription(1, Some(version)).await
    }).await;
    // let namespace_item = namespace_item.await;
    if namespace_item.is_err() { // 超时 无更新
        return Ok(Json(APIResponse::ok()));
    }
    let namespace_item = namespace_item.unwrap();
    if namespace_item.is_none() {
        return Ok(Json(APIResponse::ok()));
    }
    Ok(Json(APIResponse::ok_data(vec![namespace_item.unwrap()])))

}
