use std::collections::HashSet;

use crate::web::{
    extract::error::APIError,
    store::dao::{app, rule, user_role},
};

use entity::{rule::Verb, users::UserLevel, UserAuth};

#[inline]
pub async fn acc_admin(auth: &UserAuth, app_id: Option<String>) -> Result<bool, APIError> {
    match auth.level {
        // 是否超级管理员
        UserLevel::Admin => Ok(true),
        UserLevel::DeptAdmin => {
            return match app_id {
                Some(id) => {
                    // same department
                    Ok(app::App
                        .get_app_department_by_id(id)
                        .await?
                        .unwrap_or_default()
                        == auth.dept_id)
                }
                None => Ok(false),
            };
        }
        _ => Ok(false),
    }
}

pub async fn accredit(auth: &UserAuth, verb: Verb, resource: &Vec<&str>) -> Result<bool, APIError> {
    if resource.len() == 0 {
        return Ok(false);
    }
    if acc_admin(auth, resource.first().and_then(|x| Some(x.to_string()))).await? {
        return Ok(true);
    }
    // 获得用户的角色ID
    let user_roles = user_role::UserRule.get_user_role(auth.id).await?;
    if user_roles.is_empty() {
        return Ok(false);
    }
    // 获取授权资源的角色ID
    let auth_roles = rule::Rule
        .get_resource_role(verb, rule::combination_resource(resource))
        .await?;
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
