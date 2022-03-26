use super::dao::{cluster, namespace};
use super::response::{APIError, APIResponse, ParamErrType};
use super::APIResult;
use super::{check, ReqJson, ReqQuery};

use axum::extract::Json;
use entity::constant::NAME_MAX_LEN;
use entity::orm::Set;
use entity::{NamespaceActive, NamespaceModel, ID};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NamespaceParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
    pub namespace: Option<String>,
}

pub async fn create(ReqJson(param): ReqJson<NamespaceParam>) -> APIResult<Json<APIResponse<ID>>> {
    let namespace = check::name(param.namespace, "namespace")?;
    let app_id = match param.app_id {
        Some(id) => {
            if id.len() == 0 || id.len() > NAME_MAX_LEN {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            id
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

    // 查看当前 app_id namespace 是否存在
    let id: Option<ID> = cluster::is_exist(&app_id, &cluster).await?;
    if id.is_none() {
        return Err(APIError::new_param_err(
            ParamErrType::NotExist,
            "app_id, cluster",
        ));
    }

    // 查看是否已存在此 namespace
    let id: Option<ID> = namespace::is_exist(&app_id, &cluster, &namespace).await?;
    if id.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "namespace"));
    }
    let data = NamespaceActive {
        app_id: Set(app_id),
        cluster_name: Set(cluster),
        namespace: Set(namespace),
        ..Default::default()
    };

    let id = namespace::insert(data).await?;
    Ok(Json(APIResponse::ok_data(ID::new(id))))
}

#[derive(Deserialize)]
pub struct NamespaceQueryParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

pub async fn list(
    ReqQuery(param): ReqQuery<NamespaceQueryParam>,
) -> APIResult<Json<APIResponse<Vec<NamespaceModel>>>> {
    if let Some(app_id) = &param.app_id {
        if app_id.len() != 0 && app_id.len() > 100 {
            return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
        }
    }
    if let Some(name) = &param.cluster {
        if name.len() != 0 && name.len() > 100 {
            return Err(APIError::new_param_err(ParamErrType::NotExist, "cluster"));
        }
        // 如果有 cluster_name 则 app_id 必填
        if let Some(app_id) = &param.app_id {
            if app_id.len() == 0 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
        } else {
            return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
        }
    }
    let (page, page_size) = check::page(param.page, param.page_size);
    let list: Vec<NamespaceModel> = namespace::find_by_app_cluster_all(
        param.app_id,
        param.cluster,
        (page - 1) * page_size,
        page_size,
    )
    .await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page, page_size);
    Ok(Json(rsp))
}
