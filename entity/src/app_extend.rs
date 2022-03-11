use super::common::{Premissions, Status};
use crate::utils::get_time_zone;
use crate::utils::grable_id;

use chrono::Local;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "app_extend")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "grable_id")]
    pub id: i64,
    #[sea_orm(indexed, column_type = "String(Some(100))")]
    pub app_id: String, // app 唯一 ID
    #[sea_orm(column_type = "String(Some(100))")]
    pub name: String, // app  namespace
    pub premissions: Premissions,         // 权限
    pub status: Status,                   // 状态
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
            premissions: Set(Premissions::Private),
            status: Set(Status::Normal),
            created_at: Set(Local::now().with_timezone(get_time_zone())),
            updated_at: Set(Local::now().with_timezone(get_time_zone())),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.updated_at = Set(Local::now().with_timezone(get_time_zone()));
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
