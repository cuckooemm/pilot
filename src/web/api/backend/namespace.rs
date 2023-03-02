use std::collections::HashSet;

use crate::web::api::check;
use crate::web::api::permission::accredit;
use crate::web::extract::error::{APIError, ParamErrType, ForbiddenType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::{rule, Dao};
use crate::web::APIResult;

use axum::extract::State;
use axum::Extension;
use entity::Scope;
use entity::model::{
    namespace::{NamespaceInfo, NamespaceItem},
    rule::Verb,
    NamespaceActive, NamespaceModel, UserAuth,
};
use entity::orm::Set;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NamespaceParam {
    pub app: Option<String>,
    pub cluster: Option<String>,
    pub namespace: Option<String>,
    pub scope: Option<String>,
}

pub async fn create(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<NamespaceParam>,
) -> APIResult<APIResponse<NamespaceModel>> {
    let namespace = check::id_str(param.namespace, "namespace")?;
    let app = check::id_str(param.app, "app")?;
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
    let resource = vec![app.as_str(), cluster.as_str()];
    if !accredit::accredit(&auth, Verb::Create, &vec![&app, &cluster]).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Operate,
            &resource,
        ));
    }
    // 查看当前 app_id cluster 是否存在
    if !dao.cluster.is_exist(app.clone(), cluster.clone()).await? {
        return Err(APIError::param_err(ParamErrType::NotExist, "app, cluster"));
    }

    // 查看是否已存在此 namespace
    if dao
        .namespace
        .is_exist(app.clone(), cluster.clone(), namespace.clone())
        .await?
    {
        return Err(APIError::param_err(ParamErrType::Exist, "namespace"));
    }

    let active = NamespaceActive {
        app: Set(app),
        cluster: Set(cluster),
        namespace: Set(namespace),
        scope: Set(scope),
        ..Default::default()
    };
    let data = dao.namespace.addition(active).await?;
    Ok(APIResponse::ok_data(data))
}

#[derive(Deserialize)]
pub struct NamespaceQueryParam {
    pub app: Option<String>,
    pub cluster: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

pub async fn list(
    ReqQuery(param): ReqQuery<NamespaceQueryParam>,
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
) -> APIResult<APIResponse<Vec<NamespaceItem>>> {
    let app = check::id_str(param.app, "app")?;
    let cluster = check::id_str(param.cluster, "cluster")?;
    let list: Vec<NamespaceItem> = dao
        .namespace
        .get_namespace_by_appcluster(app.clone(), cluster.clone())
        .await?;
    if list.is_empty() {
        return Ok(APIResponse::ok_data(list));
    }
    // 校验
    if accredit::acc_admin(&auth, Some(app.clone())).await? {
        return Ok(APIResponse::ok_data(list));
    }
    // 获取用户角色ID
    let user_roles = dao.user_role.get_user_role(auth.id).await?;
    if user_roles.is_empty() {
        // 返回空
        return Ok(APIResponse::ok_data(vec![]));
    }
    let user_role_set: HashSet<u32> = HashSet::from_iter(user_roles.into_iter());

    // 获取上级资源权限 如果有则返回
    let role = dao
        .rule
        .get_resource_role(Verb::VIEW, vec![app.clone(), cluster.clone()])
        .await?;
    for r_id in role.iter() {
        // 拥有上级资源权限角色  直接返回
        if user_role_set.contains(r_id) {
            return Ok(APIResponse::ok_data(list));
        }
    }
    // 获取此资源下级拥有View权限的所有角色
    let role = dao
        .rule
        .get_resource_prefix_role(Verb::VIEW, app.clone(), Some(cluster.clone()))
        .await?;
    if role.is_empty() {
        return Ok(APIResponse::ok_data(vec![]));
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
        return Ok(APIResponse::ok_data(vec![]));
    }

    let list: Vec<NamespaceItem> = list
        .into_iter()
        .filter(|c| rules.contains(&c.namespace))
        .collect();
    Ok(APIResponse::ok_data(list))
}

#[derive(Deserialize)]
pub struct PublucNamespaceQueryParam {
    pub namespace: Option<String>,
}
// 获取公共的namespace
pub async fn list_public(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqQuery(param): ReqQuery<PublucNamespaceQueryParam>,
) -> APIResult<APIResponse<Vec<NamespaceInfo>>> {
    let namespace = check::id_str(param.namespace, "namespace")?;
    let list = dao.namespace.get_public_namespace_info(namespace).await?;
    Ok(APIResponse::ok_data(list))
}
