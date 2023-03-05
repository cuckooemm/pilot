use crate::common::enums::{Scope, Status};

use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "namespace")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    pub app: String,
    pub cluster: String,
    pub namespace: String,
    pub scope: Scope,
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

#[derive(FromQueryResult, Serialize, Debug)]
pub struct NamespaceItem {
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    pub namespace: String,
}

#[derive(FromQueryResult, Serialize, Debug)]
pub struct NamespaceInfo {
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    pub app_id: String, // app ID
    pub cluster: String,
    pub namespace: String,
}
