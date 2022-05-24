use std::fmt::Display;

use super::common::ItemCategory;
use crate::grable_id;

use chrono::{FixedOffset, Local};
use sea_orm::{entity::prelude::*, FromQueryResult, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "publication")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "grable_id")]
    pub id: u64,
    #[sea_orm(unique)]
    #[serde(serialize_with = "grable_id")]
    pub item_id: u64,
    #[sea_orm(indexed)]
    #[serde(serialize_with = "grable_id")]
    pub namespace_id: u64,
    #[sea_orm(column_type = "String(Some(100))")]
    pub key: String,
    #[sea_orm(column_type = "Text")]
    pub value: String,
    pub category: ItemCategory,
    #[sea_orm(column_type = "String(Some(200))")]
    pub remark: String, // 发布备注
    #[sea_orm(default_value = 0)]
    pub publish_user_id: u64, // 发布者
    pub version: u64,
    pub published_at: DateTimeWithTimeZone, // 发布时间
}

#[derive(Clone, Debug, PartialEq, FromQueryResult, Deserialize, Serialize)]
pub struct Item {
    pub key: String,
    pub value: String,
    pub category: ItemCategory,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "k: {}, v: {}, c: {}",
            self.key,
            self.value,
            self.category.to_string()
        )
    }
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
            publish_user_id: Set(0), // TODO 发布者ID
            published_at: Set(Local::now().with_timezone(&FixedOffset::east(8 * 3600))),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.published_at = Set(Local::now().with_timezone(&FixedOffset::east(8 * 3600)));
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
