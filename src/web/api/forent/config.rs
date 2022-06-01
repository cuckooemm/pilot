use std::time::Duration;

use super::dao::{cluster, namespace};
use crate::web::extract::utils;
use crate::web::store::cache::{CacheItem, NamespaceItem};
use crate::web::{
    extract::{
        query::ReqQuery,
        response::{APIError, ApiResponse, ParamErrType},
    },
    APIResult,
};

use axum::extract::Extension;
use axum::Json;
use serde::{Deserialize, Serialize};
use tokio::time;

#[derive(Serialize, Deserialize, Debug)]
pub struct DescParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
    pub namespace: Option<String>,
    pub secret: Option<String>,
    pub version: Option<u64>,
    pub timeout: Option<u64>,
}

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

// 全量获取配置数据
pub async fn description(
    ReqQuery(param): ReqQuery<DescParam>,
    Extension(cache): Extension<CacheItem>,
) -> APIResult<Json<ApiResponse<NamespaceItem>>> {
    let app_id = match param.app_id {
        Some(app_id) => {
            if app_id.len() == 0 || app_id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            app_id
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "app_id")),
    };
    let cluster = match param.cluster {
        Some(cluster) => {
            if cluster.len() == 0 || cluster.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "cluster"));
            }
            cluster
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "cluster")),
    };
    let namespace = match &param.namespace {
        Some(ns) => {
            if ns.len() == 0 || ns.len() > 100 {
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
    let namespace_id =
        namespace::get_namespace_id(app_id.clone(), cluster.clone(), namespace.clone()).await?;
    if namespace_id.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
    }
    // 如果有没有获取到 namespace_id 的 namespace, 根据扩展 namespace 字段,寻找关联的 app_namespace
    // 默认如果此 app_id 包含同名 namespace, 则优先使用本 app_id 的 namespace， 覆盖关联的 app_namespace

    // 监听namespace
    let namespace_item = time::timeout(
        Duration::from_secs(5),
        cache.subscription(namespace_id.unwrap(), None),
    )
    .await;
    if namespace_item.is_err() {
        // 超时 无更新
        return Ok(Json(ApiResponse::ok()));
    }
    let namespace_item = namespace_item.unwrap();
    if namespace_item.is_none() {
        return Ok(Json(ApiResponse::ok()));
    }
    Ok(Json(ApiResponse::ok_data(namespace_item.unwrap())))
}

// 阻塞链接, 仅更新时返回数据
pub async fn notifaction(
    ReqQuery(param): ReqQuery<DescParam>,
    Extension(cache): Extension<CacheItem>,
) -> APIResult<Json<ApiResponse<NamespaceItem>>> {
    let app_id = match param.app_id {
        Some(app_id) => {
            if app_id.len() == 0 || app_id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            app_id
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "app_id")),
    };
    let cluster = match param.cluster {
        Some(cluster) => {
            if cluster.len() == 0 || cluster.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "cluster"));
            }
            cluster
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "cluster")),
    };
    let namespace = match &param.namespace {
        Some(ns) => {
            if ns.len() == 0 || ns.len() > 100 {
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
            if version == 0 {
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
    // 获取到 namespace_id
    let namespace_id =
        namespace::get_namespace_id(app_id.clone(), cluster.clone(), namespace.clone()).await?;
    if namespace_id.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
    }

    let namespace_item = time::timeout(
        timeout,
        cache.subscription(namespace_id.unwrap(), Some(version)),
    )
    .await;
    // let namespace_item = namespace_item.await;
    if namespace_item.is_err() {
        // 超时 无更新
        return Ok(Json(ApiResponse::ok()));
    }
    let namespace_item = namespace_item.unwrap();
    if namespace_item.is_none() {
        return Ok(Json(ApiResponse::ok()));
    }
    Ok(Json(ApiResponse::ok_data(namespace_item.unwrap())))
}
