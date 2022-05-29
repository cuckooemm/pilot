use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_favorite")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "super::grable_id")]
    pub id: u64, // 用户ID
    pub user_id: u32,                     // 登录用户名
    pub app_id: u32,                      // 邮箱
    pub deleted_at: u64,                  // 删除时间
    pub created_at: DateTimeWithTimeZone, // 创建时间
    pub updated_at: DateTimeWithTimeZone, // 更新时间
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    App,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::App => Entity::belongs_to(super::AppEntity)
                .from(Column::AppId)
                .to(super::AppColumn::Id)
                .into(),
        }
    }
}
impl Related<super::AppEntity> for Entity {
    fn to() -> RelationDef {
        Relation::App.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
