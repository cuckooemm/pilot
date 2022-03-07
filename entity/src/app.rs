use super::common::Status;
use super::TZ_CN;
use super::grable_id;

use chrono::Local;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "application")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[serde(serialize_with = "grable_id")]
    pub id: i64,
    #[sea_orm(unique)]
    #[sea_orm(column_type = "String(Some(100))")]
    pub app_id: String, // app 唯一 ID
    #[sea_orm(column_type = "String(Some(100))")]
    pub name: String, // app name
    pub org_id: i32, // 组织 ID
    #[sea_orm(column_type = "String(Some(100))")]
    pub org_name: String, // 组织名
    #[sea_orm(column_type = "String(Some(100))")]
    pub owner_name: String, // 拥有者
    #[sea_orm(column_type = "String(Some(100))")]
    pub owner_email: String, // 拥有者邮箱
    pub status: Status, // 状态
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

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            org_id: Set(0),
            org_name: Set("".to_owned()),
            owner_name: Set("".to_owned()),
            owner_email: Set("".to_owned()),
            status: Set(Status::Normal),
            created_at: Set(Local::now().with_timezone(&TZ_CN)),
            updated_at: Set(Local::now().with_timezone(&TZ_CN)),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.updated_at = Set(Local::now().with_timezone(&TZ_CN));
        Ok(self)
    }

    /// Will be triggered after insert / update
    fn after_save(model: Model, _insert: bool) -> Result<Model, DbErr> {
        Ok(model)
    }

    /// Will be triggered before delete
    fn before_delete(self) -> Result<Self, DbErr> {
        Ok(self)
    }

    /// Will be triggered after delete
    fn after_delete(self) -> Result<Self, DbErr> {
        Ok(self)
    }
}
