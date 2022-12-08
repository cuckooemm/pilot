use std::collections::HashSet;

use super::APIResult;
use crate::web::api::check;
use crate::web::api::permission::accredit;
use crate::web::extract::error::{APIError, ForbiddenType, ParamErrType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::{rule, Dao};

use axum::extract::State;
use axum::Extension;
use chrono::Local;
use entity::cluster::ClusterItem;
use entity::enums::Status;
use entity::orm::{ActiveModelTrait, IntoActiveModel, Set};
use entity::rule::Verb;
use entity::{ClusterActive, ClusterModel, UserAuth};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

const SECRET_LEN: usize = 36;

#[derive(Deserialize, Debug)]
pub struct ClusterParam {
    pub app: Option<String>,
    pub cluster: Option<String>,
    pub describe: Option<String>,
}

// create app cluster
pub async fn create(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<ClusterParam>,
) -> APIResult<APIResponse<ClusterModel>> {
    let cluster = check::id_str(param.cluster, "cluster")?;
    let app = check::id_str(param.app, "app")?;
    let describe = if let Some(desc) = param.describe {
        if desc.len() > 200 {
            return Err(APIError::param_err(ParamErrType::Max(200), "describe"));
        }
        desc
    } else {
        String::default()
    };
    if !dao.app.is_exist(app.clone()).await? {
        return Err(APIError::param_err(ParamErrType::NotExist, "app_id"));
    }
    let resouce = vec![app.as_str()];
    if !accredit::accredit(&auth, entity::rule::Verb::Create, &resouce).await? {
        return Err(APIError::forbidden_resource(
            crate::web::extract::error::ForbiddenType::Operate,
            &resouce,
        ));
    }
    if dao.cluster.is_exist(app.clone(), cluster.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "cluster"));
    }

    let active = ClusterActive {
        app: Set(app),
        cluster: Set(cluster),
        secret: Set(general_rand_secret()),
        describe: Set(describe),
        ..Default::default()
    };
    let data = dao.cluster.addition(active).await?;
    Ok(APIResponse::ok_data(data))
}

#[derive(Deserialize, Debug)]
pub struct EditParam {
    pub id: Option<String>,
    pub reset_secret: Option<bool>,
    pub descibe: Option<String>,
    pub status: Option<String>,
}

pub async fn edit(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<EditParam>,
) -> APIResult<APIResponse<ClusterModel>> {
    let id = check::id_decode::<u64>(param.id, "id")?;
    let cluster = dao
        .cluster
        .get_cluster_by_id(id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "id"))?;
    let resource = vec![cluster.app.as_str(), cluster.cluster.as_str()];
    if !accredit::accredit(&auth, entity::rule::Verb::Modify, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Operate,
            &resource,
        ));
    }
    let mut active = cluster.clone().into_active_model();
    if param.reset_secret.unwrap_or_default() {
        active.secret = Set(general_rand_secret());
    }
    if let Some(desc) = param.descibe {
        if desc != cluster.describe {
            if desc.len() > 200 {
                return Err(APIError::param_err(ParamErrType::Max(200), "describe"));
            }
            active.describe = Set(desc);
        }
    }
    if let Some(status) = param.status.and_then(|s| s.try_into().ok()) {
        if status != cluster.status {
            active.status = Set(status);
        }
    }
    if !active.is_changed() {
        return Ok(APIResponse::ok_data(cluster));
    }
    let data = dao.cluster.update(active).await?;
    Ok(APIResponse::ok_data(data))
}

#[derive(Deserialize, Debug)]
pub struct ClusterQueryParam {
    pub app: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

pub async fn list(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqQuery(param): ReqQuery<ClusterQueryParam>,
) -> APIResult<APIResponse<Vec<ClusterItem>>> {
    let app = check::id_str(param.app, "app")?;

    // 获取内容
    let list = dao.cluster.find_cluster_by_app(app.clone()).await?;
    // 无内容直接返回
    if list.is_empty() {
        return Ok(APIResponse::ok_data(list));
    }
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
        .get_resource_role(Verb::VIEW, vec![app.clone()])
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
        .get_resource_prefix_role(Verb::VIEW, app.clone(), None)
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
        return Ok(APIResponse::ok_data(vec![]));
    }

    let list: Vec<ClusterItem> = list
        .into_iter()
        .filter(|c| rules.contains(&c.name))
        .collect();
    Ok(APIResponse::ok_data(list))
}

// 生成密钥
fn general_rand_secret() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SECRET_LEN)
        .map(char::from)
        .collect()
}
