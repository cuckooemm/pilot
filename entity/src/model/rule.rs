use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "rule")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: u64,
    pub verb: Verb,
    pub resource: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RoleRule,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::RoleRule => Entity::has_many(super::RoleRuleEntity).into(),
        }
    }
}

impl Related<super::RoleRuleEntity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleRule.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "SmallUnsigned")]
pub enum Verb {
    // 创建
    #[sea_orm(string_value = "Create")]
    Create,
    // 修改
    #[sea_orm(string_value = "Modify")]
    Modify,
    // 查看
    #[sea_orm(string_value = "View")]
    VIEW,
    // 发布
    #[sea_orm(string_value = "Publish")]
    Publish,
}
