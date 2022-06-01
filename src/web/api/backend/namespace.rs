use std::collections::HashSet;

use super::dao::{cluster, namespace};
use super::response::{APIError, APIResponse, ParamErrType};
use super::APIResult;
use super::{check, ReqJson, ReqQuery};
use crate::web::api::permission::accredit;
use crate::web::extract::jwt::Claims;
use crate::web::store::dao::{rule, user_role};

use axum::extract::Json;
use entity::namespace::{NamespaceInfo, NamespaceItem};
use entity::orm::Set;
use entity::rule::Verb;
use entity::{NamespaceActive, Scope, ID};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NamespaceParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
    pub namespace: Option<String>,
    pub scope: Option<String>,
}

pub async fn create(
    ReqJson(param): ReqJson<NamespaceParam>,
    auth: Claims,
) -> APIResult<Json<APIResponse<ID>>> {
    let namespace = check::id_str(param.namespace, "namespace")?;
    let app_id = check::id_str(param.app_id, "app_id")?;
    let scope = Scope::from(param.scope.unwrap_or_default());
    let cluster = match scope {
        // 私有的 集群字段必填
        Scope::Private => check::id_str(param.cluster, "cluster"),
        // 公共的 集群字段可不填  如果填仅校验
        Scope::Public => {
            if param.cluster.is_some() {
                check::id_str_rule(param.cluster.unwrap(), "cluster")
            } else {
                Ok(String::from("global"))
            }
        }
    }?;
    // 校验权限
    if !accredit::accredit(&auth, entity::rule::Verb::Create, vec![&app_id, &cluster]).await? {
        return Err(APIError::new_permission_forbidden());
    }
    // 查看当前 app_id cluster 是否存在
    if !cluster::is_exist(app_id.clone(), cluster.clone()).await? {
        return Err(APIError::new_param_err(
            ParamErrType::NotExist,
            "app_id, cluster",
        ));
    }

    // 查看是否已存在此 namespace
    if namespace::is_exist(app_id.clone(), cluster.clone(), namespace.clone()).await? {
        return Err(APIError::new_param_err(ParamErrType::Exist, "namespace"));
    }

    let data = NamespaceActive {
        app_id: Set(app_id),
        cluster: Set(cluster),
        namespace: Set(namespace),
        scope: Set(scope),
        creator_user: Set(auth.user_id),
        ..Default::default()
    };

    namespace::add(data).await?;
    Ok(Json(APIResponse::ok()))
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
    auth: Claims,
) -> APIResult<Json<APIResponse<Vec<NamespaceItem>>>> {
    let app_id = check::id_str(param.app_id, "app_id")?;
    let cluster = check::id_str(param.cluster, "cluster")?;
    let list: Vec<NamespaceItem> =
        namespace::get_namespace_by_appcluster(app_id.clone(), cluster.clone()).await?;
    if list.is_empty() {
        return Ok(Json(APIResponse::ok_data(list)));
    }
    // 校验
    if accredit::acc_admin(&auth, Some(app_id.clone())) {
        return Ok(Json(APIResponse::ok_data(list)));
    }
    // 获取用户角色ID
    let user_roles = user_role::get_user_role(auth.user_id).await?;
    if user_roles.is_empty() {
        // 返回空
        return Ok(Json(APIResponse::ok_data(vec![])));
    }
    let user_role_set: HashSet<u32> = HashSet::from_iter(user_roles.into_iter());

    // 获取上级资源权限 如果有则返回
    let role = rule::get_resource_role(Verb::VIEW, vec![app_id.clone(), cluster.clone()]).await?;
    for r_id in role.iter() {
        // 拥有上级资源权限角色  直接返回
        if user_role_set.contains(r_id) {
            return Ok(Json(APIResponse::ok_data(list)));
        }
    }
    // 获取此资源下级拥有View权限的所有角色
    let role =
        rule::get_resource_prefix_role(Verb::VIEW, app_id.clone(), Some(cluster.clone())).await?;
    if role.is_empty() {
        return Ok(Json(APIResponse::ok_data(vec![])));
    }

    let mut rules = HashSet::with_capacity(role.len());
    for r in role.into_iter() {
        if !user_role_set.contains(&r.role_id) {
            // 用户无此角色
            continue;
        }
        let mut rk = rule::parse_resource_kind(r.resource);
        // 仅3级权限资源有权限 1 2级资源权限之前已经校验
        // app_id  app_id/cluster app_id/cluster/namespace 拥有权限
        if rk.len() != 3 {
            // 下级资源权限过滤
            continue;
        }
        // 拥有权限的 cluster
        rk.pop().map(|x| rules.insert(x));
    }
    if rules.is_empty() {
        // 无相关权限
        return Ok(Json(APIResponse::ok_data(vec![])));
    }

    let list: Vec<NamespaceItem> = list
        .into_iter()
        .filter(|c| rules.contains(&c.namespace))
        .collect();
    Ok(Json(APIResponse::ok_data(list)))
}

#[derive(Deserialize)]
pub struct PublucNamespaceQueryParam {
    pub namespace: Option<String>,
}
// 获取公共的namespace
pub async fn list_public(
    ReqQuery(param): ReqQuery<PublucNamespaceQueryParam>,
    auth: Claims,
) -> APIResult<Json<APIResponse<Vec<NamespaceInfo>>>> {
    let namespace = check::id_str(param.namespace, "namespace")?;

    let list = namespace::get_public_namespace_info(namespace).await?;
    Ok(Json(APIResponse::ok_data(list)))
}
