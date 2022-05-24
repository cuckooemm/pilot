use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "cluster")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: u64,
    pub app_id: String, // app 唯一 ID
    pub name: String,   // cluster name
    pub secret: String, // 连接 secret
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

#[derive(FromQueryResult)]
pub struct SecretData {
    pub secret: String,
}

#[derive(FromQueryResult, Serialize, Debug)]
pub struct ClusterItem {
    #[serde(serialize_with = "super::grable_id")]
    pub id: u64,
    pub name: String,
}
