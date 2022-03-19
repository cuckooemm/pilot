use std::time::Duration;

use super::dao::{cluster, namespace};
use crate::web::extract::utils;
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
            if ns.len() == 0 || ns.len() > NAME_MAX_LEN {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
            }
            ns
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "namespace")),
    };
    let encode_secret = match param.secret {
        Some(secret) => {
            if secret.len() != 32 {
                return Err(APIError::new_param_err(ParamErrType::Invalid, "secret"));
            }
            secret
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "secret")),
    };
    // 查看 cluster 是否存在 且获取到 secret
    match cluster::get_secret_by_cluster(&app_id, &cluster).await? {
        Some(secret) => {
            // 校验secret
            if encode_secret
                != utils::hex_md5(format!("{}-{}-{}", &app_id, &cluster, &secret.secret))
            {
                return Err(APIError::new_param_err(ParamErrType::Invalid, "secret"));
            }
        }
        None => return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id")),
    };

    // 获取到 namespace_id
    let namespace_id = namespace::is_exist(&app_id, &cluster, namespace).await?;
    if namespace_id.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
    }
    // 如果有没有获取到 namespace_id 的 namespace, 根据扩展 namespace 字段,寻找关联的 app_namespace
    // 默认如果此 app_id 包含同名 namespace, 则优先使用本 app_id 的 namespace， 覆盖关联的 app_namespace

    // 监听namespace
    let namespace_item = time::timeout(Duration::from_secs(1), cache.subscription(namespace_id.unwrap().id as u64, None)).await;
    if namespace_item.is_err() {
        // 超时 无更新
        return Ok(Json(APIResponse::ok()));
    }
    let namespace_item = namespace_item.unwrap();
    if namespace_item.is_none() {
        return Ok(Json(APIResponse::ok()));
    }
    Ok(Json(APIResponse::ok_data(vec![namespace_item.unwrap()])))
}

// 阻塞链接, 仅更新时返回数据
pub async fn notifaction(
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
            if ns.len() == 0 || ns.len() > NAME_MAX_LEN {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
            }
            ns
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "namespace")),
    };
    let encode_secret = match param.secret {
        Some(secret) => {
            if secret.len() != 32 {
                return Err(APIError::new_param_err(ParamErrType::Invalid, "secret"));
            }
            secret
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "secret")),
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
            if tm == 0 || tm > DEFAULT_TIMEOUT.as_secs_f64() as u64 {
                DEFAULT_TIMEOUT
            } else {
                Duration::from_secs(tm)
            }
        }
        None => DEFAULT_TIMEOUT,
    };
    // 查看 cluster 是否存在 且获取到 secret
    match cluster::get_secret_by_cluster(&app_id, &cluster).await? {
        Some(secret) => {
            // 校验secret
            if encode_secret
                != utils::hex_md5(format!("{}-{}-{}", &app_id, &cluster, &secret.secret))
            {
                return Err(APIError::new_param_err(ParamErrType::Invalid, "secret"));
            }
        }
        None => return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id")),
    };


    let namespace_item = time::timeout(timeout, cache.subscription(1, Some(version))).await;
    // let namespace_item = namespace_item.await;
    if namespace_item.is_err() {
        // 超时 无更新
        return Ok(Json(APIResponse::ok()));
    }
    let namespace_item = namespace_item.unwrap();
    if namespace_item.is_none() {
        return Ok(Json(APIResponse::ok()));
    }
    Ok(Json(APIResponse::ok_data(vec![namespace_item.unwrap()])))
}
