use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::enums::Status;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "departments")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "super::confuse")]
    pub id: u32,
    pub name: String,
    pub status: Status,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
