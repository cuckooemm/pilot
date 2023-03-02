use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::enums::Status;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "collection")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    pub user_id: u32,
    pub app_id: u32,
    pub status: Status,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
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
