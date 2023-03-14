use crate::common::enums::ItemCategory;
use crate::common::enums::Status;

use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    #[serde(serialize_with = "crate::confuse")]
    pub namespace_id: u64,
    pub key: String,
    pub value: String,
    pub category: ItemCategory,
    pub remark: String,
    pub version: u64,
    pub status: Status,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
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
    pub status: Status,
}