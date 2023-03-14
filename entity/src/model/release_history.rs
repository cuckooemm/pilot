use crate::ItemCategory;
use super::{item::ItemData, release::ItemDesc};

use sea_orm::{entity::prelude::*, FromQueryResult, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "release_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    #[serde(serialize_with = "crate::confuse")]
    pub namespace_id: u64,
    #[serde(serialize_with = "crate::confuse")]
    pub release_version: u64,
    #[serde(serialize_with = "crate::confuse")]
    pub item_id: u64,
    pub action: ReleaseAction,
    pub key: String,
    pub value: String,
    pub category: ItemCategory,
    pub version: u64,
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
pub struct HistoryItem {
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    #[serde(serialize_with = "crate::confuse")]
    pub release_id: u64,
    pub change: String,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "u8", db_type = "TinyUnsigned")]
pub enum ReleaseAction {
    #[sea_orm(num_value = 0)]
    Add,
    #[sea_orm(num_value = 2)]
    Modify,
    #[sea_orm(num_value = 10)]
    Remove,
}

impl IntoActiveModel<ActiveModel> for ItemData {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            namespace_id: Set(self.namespace_id),
            item_id: Set(self.id),
            action: Set(ReleaseAction::Add),
            key: Set(self.key),
            value: Set(self.value),
            category: Set(self.category),
            version: Set(self.version),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<ActiveModel> for ItemDesc {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            item_id: Set(self.id),
            action: Set(ReleaseAction::Add),
            key: Set(self.key),
            value: Set(self.value),
            category: Set(self.category),
            version: Set(self.version),
            ..Default::default()
        }
    }
}
