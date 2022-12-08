use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::enums::Status;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "app_extend")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: u64,
    pub app: String, // app 唯一 ID
    pub namespace_id: u64,
    pub namespace_name: String, // app  namespace
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
