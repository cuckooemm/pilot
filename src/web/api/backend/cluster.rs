use std::collections::HashSet;

use super::dao::cluster;
use super::response::{APIError, ApiResponse, ParamErrType};
use super::APIResult;
use super::{check, ReqJson, ReqQuery};
use crate::web::api::permission::accredit;
use crate::web::extract::response::Empty;
use crate::web::store::dao::{app, rule, user_role};

use axum::extract::Json;
use axum::Extension;
use entity::cluster::ClusterItem;
use entity::orm::Set;
use entity::rule::Verb;
use entity::{ClusterActive, UserAuth, ID};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

const SECRET_LEN: usize = 36;

#[derive(Deserialize, Debug)]
pub struct ClusterParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
}

// 创建app集群
pub async fn create(
    ReqJson(param): ReqJson<ClusterParam>,
    Extension(auth): Extension<UserAuth>,
) -> APIResult<Json<ApiResponse<Empty>>> {
    // check param
    let cluster = check::id_str(param.cluster, "cluster")?;
    let app_id = check::id_str(param.app_id, "app_id")?;
    if !app::is_exist(app_id.clone()).await? {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
    }
    // 校验权限
    if !accredit::accredit(&auth, entity::rule::Verb::Create, vec![&app_id]).await? {
        return Err(APIError::new_permission_forbidden());
    }

    // 查看当前 app_id cluster_name是否存在
    if cluster::is_exist(app_id.clone(), cluster.clone()).await? {
        return Err(APIError::new_param_err(ParamErrType::Exist, "cluster"));
    }

    let data = ClusterActive {
        app_id: Set(app_id),
        name: Set(cluster),
        secret: Set(general_rand_secret()),
        creator_user: Set(auth.id),
        ..Default::default()
    };
    cluster::add(data).await?;
    Ok(Json(ApiResponse::ok()))
}

#[derive(Deserialize, Debug)]
pub struct EditParam {
    pub id: Option<String>,
    pub secret: Option<String>,
}
pub async fn edit(
    ReqJson(param): ReqJson<EditParam>,
    Extension(auth): Extension<UserAuth>,
) -> APIResult<Json<ApiResponse<Empty>>> {
    Ok(Json(ApiResponse::ok()))
}

// 重置密钥接口
pub async fn reset_secret(
    ReqJson(param): ReqJson<ClusterParam>,
    Extension(auth): Extension<UserAuth>,
) -> APIResult<Json<ApiResponse<Empty>>> {
    let cluster = check::id_str(param.cluster, "cluster")?;
    let app_id = check::id_str(param.app_id, "app_id")?;
    // 校验权限
    if !accredit::accredit(&auth, entity::rule::Verb::Modify, vec![&app_id, &cluster]).await? {
        return Err(APIError::new_permission_forbidden());
    }
    let id = cluster::find_app_cluster(app_id, cluster)
        .await?
        .unwrap_or_default();
    if id == 0 {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "cluster"));
    }
    let active = ClusterActive {
        secret: Set(general_rand_secret()),
        ..Default::default()
    };
    cluster::update_by_id(active, id).await?;
    Ok(Json(ApiResponse::ok()))
}

#[derive(Deserialize, Debug)]
pub struct ClusterQueryParam {
    pub app_id: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

pub async fn list(
    ReqQuery(param): ReqQuery<ClusterQueryParam>,
    Extension(auth): Extension<UserAuth>,
) -> APIResult<Json<ApiResponse<Vec<ClusterItem>>>> {
    let app_id = check::id_str(param.app_id, "app_id")?;

    // 获取内容
    let list = cluster::find_cluster_by_app(app_id.clone()).await?;
    // 无内容直接返回
    if list.is_empty() {
        return Ok(Json(ApiResponse::ok_data(list)));
    }
    if accredit::acc_admin(&auth, Some(app_id.clone())) {
        return Ok(Json(ApiResponse::ok_data(list)));
    }
    // 获取用户角色ID
    let user_roles = user_role::get_user_role(auth.id).await?;
    if user_roles.is_empty() {
        // 返回空
        return Ok(Json(ApiResponse::ok_data(vec![])));
    }
    let user_role_set: HashSet<u32> = HashSet::from_iter(user_roles.into_iter());

    // 获取上级资源权限 如果有则返回
    let role = rule::get_resource_role(Verb::VIEW, vec![app_id.clone()]).await?;
    for r_id in role.iter() {
        // 拥有上级资源权限角色  直接返回
        if user_role_set.contains(r_id) {
            return Ok(Json(ApiResponse::ok_data(list)));
        }
    }

    // 获取此资源下级拥有View权限的所有角色
    let role = rule::get_resource_prefix_role(Verb::VIEW, app_id.clone(), None).await?;
    if role.is_empty() {
        return Ok(Json(ApiResponse::ok_data(vec![])));
    }

    let mut rules = HashSet::with_capacity(role.len());
    for r in role.into_iter() {
        if !user_role_set.contains(&r.role_id) {
            // 用户无此角色
            continue;
        }
        let mut rk = rule::parse_resource_kind(r.resource);
        // 仅2级权限资源有权限 1级资源权限之前已经校验
        // app_id  app_id/cluster 拥有权限
        // app_id/cluster/namespace 没有权限
        if rk.len() != 2 {
            // 下级资源权限过滤
            continue;
        }
        // 拥有权限的 cluster
        rk.pop().map(|x| rules.insert(x));
    }
    if rules.is_empty() {
        // 无相关权限
        return Ok(Json(ApiResponse::ok_data(vec![])));
    }

    let list: Vec<ClusterItem> = list
        .into_iter()
        .filter(|c| rules.contains(&c.name))
        .collect();
    Ok(Json(ApiResponse::ok_data(list)))
}

// 生成密钥
fn general_rand_secret() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SECRET_LEN)
        .map(char::from)
        .collect()
}
