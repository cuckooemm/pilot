use super::slaver;

use entity::orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::rule::Verb;
use entity::user_role::{RoleResource, UserRoleID};
use entity::{RoleRuleColumn, RoleRuleEntity, RuleColumn, RuleEntity};

const RESOURCE_PAT: &str = ".";

#[inline]
pub fn combination_resource(resource: Vec<&str>) -> Vec<String> {
    let mut resources = Vec::with_capacity(resource.len());
    for (idx, &r) in resource.iter().enumerate() {
        if idx == 0 {
            resources.push(r.to_string());
        } else {
            unsafe {
                resources
                    .push(resources.get_unchecked(idx - 1).clone() + RESOURCE_PAT + &r.to_string());
            }
        }
    }
    resources
}

#[inline]
pub fn parse_resource_kind_len(resource: &str) -> usize {
    resource.split(RESOURCE_PAT).count()
}

#[inline]
pub fn parse_resource_kind(resource: String) -> Vec<String> {
    resource
        .split(RESOURCE_PAT)
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}

pub async fn get_resource_prefix_role(
    verb: Verb,
    mut app_id: String,
    cluster: Option<String>,
) -> Result<Vec<RoleResource>, DbErr> {
    // 添加尾部分隔符 避免获取到相同前缀资源
    app_id.push_str(RESOURCE_PAT);
    if cluster.is_some() {
        app_id.push_str(cluster.unwrap().as_str());
        app_id.push_str(RESOURCE_PAT);
    }

    RuleEntity::find()
        .select_only()
        .column(RoleRuleColumn::RoleId)
        .column(RuleColumn::Resource)
        .left_join(RoleRuleEntity)
        .filter(RuleColumn::Resource.starts_with(&app_id))
        .filter(RuleColumn::Verb.eq(verb))
        .filter(RuleColumn::DeletedAt.eq(0_u64))
        .filter(RoleRuleColumn::DeletedAt.eq(0_u64))
        .into_model::<RoleResource>()
        .all(slaver())
        .await
}

pub async fn get_resource_role(verb: Verb, resource: Vec<String>) -> Result<Vec<u32>, DbErr> {
    let role_ids = RuleEntity::find()
        .select_only()
        .column(RoleRuleColumn::RoleId)
        .left_join(RoleRuleEntity)
        .filter(RuleColumn::Resource.is_in(resource))
        .filter(RuleColumn::Verb.eq(verb))
        .filter(RuleColumn::DeletedAt.eq(0_u64))
        .filter(RoleRuleColumn::DeletedAt.eq(0_u64))
        .into_model::<UserRoleID>()
        .all(slaver())
        .await?
        .into_iter()
        .map(|id| id.role_id)
        .collect::<Vec<u32>>();

    tracing::debug!("rule role_id {:?}", &role_ids);

    Ok(role_ids)
}
