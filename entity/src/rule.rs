use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "rule")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: u64,
    pub verb: Verb,                       // 权限类型
    pub resource: String,                 // 目标
    pub deleted_at: u64,                  // 删除时间 为0则未删除
    pub created_at: DateTimeWithTimeZone, // 创建时间
    pub updated_at: DateTimeWithTimeZone, // 更新时间
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

// impl Related<super::RoleEntity> for Entity {
    // fn to() -> RelationDef {
        // Relation::RoleRule.def()
    // }
// }


impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "SmallUnsigned")]
pub enum Verb {
    #[sea_orm(string_value = "Create")]
    Create, // 创建
    #[sea_orm(string_value = "Modify")]
    Modify, // 修改
    #[sea_orm(string_value = "View")]
    VIEW, // 查看
    #[sea_orm(string_value = "Assign")]
    ASSIGN, // 授权
    #[sea_orm(string_value = "Publish")]
    Publish, // 发布
}
