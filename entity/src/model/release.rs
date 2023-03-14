use super::item::ItemData;
use crate::ItemCategory;

use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "release")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "crate::confuse")]
    pub version: u64,
    #[serde(serialize_with = "crate::confuse")]
    pub namespace_id: u64,
    pub name: String,
    pub remark: String,
    pub configurations: String,
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
pub struct ItemDesc {
    pub id: u64,
    pub key: String,
    pub value: String,
    pub category: ItemCategory,
    pub version: u64,
}

impl From<ItemData> for ItemDesc {
    fn from(value: ItemData) -> Self {
        Self {
            id: value.id,
            key: value.key,
            value: value.value,
            category: value.category,
            version: value.version,
        }
    }
}
