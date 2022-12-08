use super::Conn;

use entity::orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::rule::Verb;
use entity::user_role::{RoleResource, UserRoleID};
use entity::{RoleRuleColumn, RoleRuleEntity, RuleColumn, RuleEntity};

const RESOURCE_PAT: &str = ".";

#[derive(Debug, Clone, Default)]
pub struct Rule;

impl Rule {
    pub async fn get_resource_prefix_role(
        &self,
        verb: Verb,
        mut app_id: String,
        cluster: Option<String>,
    ) -> Result<Vec<RoleResource>, DbErr> {
        // 添加尾部分隔符 避免获取到相同前缀资源
        app_id.push_str(RESOURCE_PAT);
        cluster.and_then::<(), _>(|c| {
            app_id.push_str(c.as_str());
            app_id.push_str(RESOURCE_PAT);
            None
        });

        RuleEntity::find()
            .select_only()
            .column(RoleRuleColumn::RoleId)
            .column(RuleColumn::Resource)
            .left_join(RoleRuleEntity)
            .filter(RuleColumn::Resource.starts_with(&app_id))
            .filter(RuleColumn::Verb.eq(verb))
            .into_model::<RoleResource>()
            .all(Conn::conn().slaver())
            .await
    }

    pub async fn get_resource_role(
        &self,
        verb: Verb,
        resource: Vec<String>,
    ) -> Result<Vec<u32>, DbErr> {
        let role_ids = RuleEntity::find()
            .select_only()
            .column(RoleRuleColumn::RoleId)
            .left_join(RoleRuleEntity)
            .filter(RuleColumn::Resource.is_in(resource.clone()))
            .filter(RuleColumn::Verb.eq(verb))
            .into_model::<UserRoleID>()
            .all(Conn::conn().slaver())
            .await?
            .into_iter()
            .map(|id| id.role_id)
            .collect::<Vec<u32>>();

        tracing::debug!(
            "find the role {:?} that contains resource {}:{:?}",
            &role_ids,
            verb,
            resource
        );

        Ok(role_ids)
    }
}

#[inline]
pub fn combination_resource(resource: &Vec<&str>) -> Vec<String> {
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
