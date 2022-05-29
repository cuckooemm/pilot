use super::common::ItemCategory;

use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "super::grable_id")]
    pub id: u64,
    #[serde(serialize_with = "super::grable_id")]
    pub namespace_id: u64,
    pub key: String,
    pub value: String,
    pub category: ItemCategory,
    pub remark: String, // 注释
    pub version: u64,
    pub modify_user_id: u32, // 最后修改人
    pub deleted_at: u64,
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

#[derive(FromQueryResult, Default, Serialize, Deserialize, Debug, Clone)]
pub struct ItemDesc {
    #[serde(skip)]
    pub id: u64,
    pub key: String,
    pub value: String,
    pub category: ItemCategory,
    pub version: u64,
}

#[derive(FromQueryResult, Default, Serialize, Deserialize, Debug, Clone)]
pub struct ConfigItem {
    pub key: String,
    pub value: String,
    pub category: ItemCategory,
}

#[derive(FromQueryResult, Default, Debug, Clone)]
pub struct ItemData {
    pub id: u64,
    pub namespace_id: u64,
    pub key: String,
    pub value: String,
    pub category: ItemCategory,
    pub version: u64,
    pub deleted_at: u64,
}
