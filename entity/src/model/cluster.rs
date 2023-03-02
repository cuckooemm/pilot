use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

use super::enums::Status;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "cluster")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    pub app: String,
    pub cluster: String,
    pub describe: String,
    pub secret: String,
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

#[derive(FromQueryResult)]
pub struct SecretData {
    pub secret: String,
}

#[derive(FromQueryResult, Serialize, Debug)]
pub struct ClusterItem {
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    pub name: String,
    pub describe: String,
}
