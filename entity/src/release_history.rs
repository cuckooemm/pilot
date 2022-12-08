use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

use crate::enums::Status;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "release_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "super::confuse")]
    pub id: u64,
    #[serde(serialize_with = "super::confuse")]
    pub namespace_id: u64,
    pub release_id: u64,
    pub change: String,
    pub status: Status,
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
pub struct HistoryItem {
    #[serde(serialize_with = "super::confuse")]
    pub id: u64,
    #[serde(serialize_with = "super::confuse")]
    pub release_id: u64,
    pub change: String,
}

#[derive(FromQueryResult)]
pub struct HistoryNamespaceID {
    pub namespace_id: u64,
}
