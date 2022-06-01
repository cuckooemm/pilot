use super::common::Scope;

use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "namespace")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "super::confuse")]
    pub id: u64,
    pub app_id: String, // app ID
    pub cluster: String,
    pub namespace: String,
    pub scope: Scope,
    pub creator_user: u32,
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

#[derive(FromQueryResult, Serialize, Debug)]
pub struct NamespaceItem {
    #[serde(serialize_with = "super::confuse")]
    pub id: u64,
    pub namespace: String,
}

#[derive(FromQueryResult, Serialize, Debug)]
pub struct NamespaceInfo {
    #[serde(serialize_with = "super::confuse")]
    pub id: u64,
    pub app_id: String, // app ID
    pub cluster: String,
    pub namespace: String,
}
