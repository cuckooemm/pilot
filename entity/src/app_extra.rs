use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::enums::Status;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "app_extra")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: u64,
    pub app: String,
    pub namespace_id: u64,
    pub status: Status,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Namespace,
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Namespace => Entity::belongs_to(super::NamespaceEntity)
                .from(Column::NamespaceId)
                .to(super::NamespaceColumn::Id)
                .into(),
        }
    }
}

impl Related<super::NamespaceEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Namespace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
