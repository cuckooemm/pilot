use chrono::prelude::*;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "application")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed)]
    pub app_id: String, // app 唯一 ID
    pub name: String,        // app name
    pub org_id: i32,         // 组织 ID
    pub org_name: String,    // 组织名
    pub owner_name: String,  // 拥有者
    pub owner_email: String, // 拥有者邮箱
    pub created_at: i64,     // 创建时间
    pub updated_at: i64,     // 更新时间
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
            created_at: Set(Local::now().timestamp()),
            updated_at: Set(0),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.updated_at = Set(Local::now().timestamp());
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
