use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

use crate::enums::Status;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "apps")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: u32,
    pub app: String, // app 唯一 ID
    pub name: String,
    pub describe: String, // maxLen=200
    #[serde(serialize_with = "super::confuse")]
    pub dept_id: u32,
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
pub struct AppItem {
    pub app_id: String,
    pub name: String,
    pub describe: String,
}

#[derive(FromQueryResult, Default, Debug, Clone)]
pub struct DepartmentID {
    pub dept_id: u32,
}
