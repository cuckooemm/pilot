use std::collections::HashSet;

use crate::web::{
    extract::{jwt::Claims, response::APIError},
    store::dao::{rule, user_role},
};

use entity::{rule::Verb, users::UserLevel};
use serde_json::to_string;

#[inline]
pub fn acc_admin(user_level: &UserLevel, app_id: Option<String>) -> bool {
    match user_level {
        // 是否超级管理员
        UserLevel::Admin => true,
        UserLevel::OrgAdmin => {
            // 判断资源是否属同一部门
            // auth.org_id == resource.org_id
            false
        }
        // 继续向下判断
        UserLevel::Normal => false,
    }
}

pub async fn accredit(auth: &Claims, verb: Verb, resource: Vec<&str>) -> Result<bool, APIError> {
    if resource.len() == 0 {
        return Ok(false);
    }
    if acc_admin(
        &auth.user_level,
        resource.first().and_then(|x| Some(x.to_string())),
    ) {
        return Ok(true);
    }
    // 获得用户的角色ID
    let user_roles = user_role::get_user_role(auth.user_id).await?;
    if user_roles.is_empty() {
        return Ok(false);
    }
    // 获取授权资源的角色ID
    let auth_roles = rule::get_resource_role(verb, rule::combination_resource(resource)).await?;
    if auth_roles.is_empty() {
        return Ok(false);
    }
    // 判断角色是否相交
    let set: HashSet<u32> = HashSet::from_iter(auth_roles.into_iter());
    for role_id in user_roles.iter() {
        if set.contains(role_id) {
            return Ok(true);
        }
    }
    Ok(false)
}
