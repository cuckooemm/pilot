use std::fmt::{Display, Formatter};

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "application")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed)]
    pub app_id: String,      // app 唯一 ID
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
        <Self as ActiveModelTrait>::default()
    }

    fn before_save(self, insert: bool) -> Result<Self, DbErr> {
        Ok(self)
    }
    /// Will be triggered after insert / update
    fn after_save(model: Model, insert: bool) -> Result<Model, DbErr> {
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
