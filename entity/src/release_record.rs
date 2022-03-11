use super::common::ItemCategory;
use crate::grable_id;
use crate::utils::get_time_zone;

use chrono::Local;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "release_record")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "grable_id")]
    pub id: i64,
    #[sea_orm(indexed)]
    pub namespace_id: i64,
    #[sea_orm(column_type = "String(Some(100))")]
    pub key: String,
    #[sea_orm(column_type = "Text")]
    pub value: String,
    pub category: ItemCategory,
    #[sea_orm(column_type = "String(Some(200))")]
    pub remark: String, // 发布备注
    pub release_user_id: i64, // 发布者
    pub version: i64,
    pub created_at: DateTimeWithTimeZone, // 创建时间
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
            release_user_id: Set(0), // TODO 发布者ID
            created_at: Set(Local::now().with_timezone(get_time_zone())),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save(self, _insert: bool) -> Result<Self, DbErr> {
        Ok(self)
    }
    /// Will be triggered after insert / update
    fn after_save(model: Model, _insert: bool) -> Result<Model, DbErr> {
        Ok(model)
    }

    fn before_delete(self) -> Result<Self, DbErr> {
        Ok(self)
    }

    fn after_delete(self) -> Result<Self, DbErr> {
        Ok(self)
    }
}
