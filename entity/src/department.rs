use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "department")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "super::confuse")]
    pub id: u32,
    pub name: String, // 部门名
    #[serde(
        serialize_with = "super::format_time",
        skip_serializing_if = "super::is_zero"
    )]
    pub deleted_at: u64, // 删除时间 为0则未删除
    pub created_at: DateTimeWithTimeZone, // 创建时间
    pub updated_at: DateTimeWithTimeZone, // 更新时间
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
