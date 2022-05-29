use crate::grable_id;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "role_rule")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "grable_id")]
    pub id: u64,
    pub role_id: u32,                     // 角色ID
    pub rule_id: u64,                     // 权限ID
    pub deleted_at: u64,                  // 删除时间
    pub created_at: DateTimeWithTimeZone, // 创建时间
    pub updated_at: DateTimeWithTimeZone, // 更新时间
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Role,
    Rule,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Role => Entity::belongs_to(super::RoleEntity)
                .from(Column::RoleId)
                .to(super::RoleColumn::Id)
                .into(),
            Self::Rule => Entity::belongs_to(super::RuleEntity)
                .from(Column::RuleId)
                .to(super::RuleColumn::Id)
                .into(),
        }
    }
}

impl Related<super::RuleEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Rule.def()
    }
}

impl Related<super::RoleEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
