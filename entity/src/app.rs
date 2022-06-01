use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "app")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: u32,
    pub app_id: String,                   // app 唯一 ID
    pub name: String,                     // app name
    pub dept_id: u32,                     // 部门 ID
    pub dept_name: String,                // 部门名称
    pub creator_user: u32,                // 创建者ID
    pub deleted_at: u64,                  // 删除时间 为0则未删除
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
pub struct AppItem {
    pub app_id: String,
    pub name: String,
}
