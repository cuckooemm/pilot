use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

use crate::common::enums::Status;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "release")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    #[serde(serialize_with = "crate::confuse")]
    pub namespace_id: u64,
    pub name: String,
    pub remark: String, // 备注
    pub configurations: String,
    pub is_abandoned: Effective, // 是否有效
    pub status: Status,
    pub created_at: DateTimeWithTimeZone, // 创建时间
    pub updated_at: DateTimeWithTimeZone, // 更新时间
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "u8", db_type = "TinyUnsigned")]
pub enum Effective {
    #[sea_orm(num_value = 0)]
    Valid,
    #[sea_orm(num_value = 1)]
    Invalid,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct ReleaseItemVersion {
    pub id: u64,
    pub version: u64,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseConfig {
    pub id: u64,
    pub configurations: String,
}
