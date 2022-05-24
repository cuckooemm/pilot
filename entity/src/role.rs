use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: u32,
    pub name: String, // app name
    pub deleted_at: u64, // 删除时间 为0则未删除
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
            Self::RoleRule => Entity::belongs_to(super::RoleRuleEntity)
            .from(Column::Id)
            .to(super::RoleRuleColumn::RoleId)
            .into(),
        }
    }
}
impl Related<super::RoleRuleEntity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleRule.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
