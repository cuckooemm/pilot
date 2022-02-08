use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(indexed)]
    pub app_id: String,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub version: i64,
    pub created_at: i64,
    pub updated_at: i64,
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
        ActiveModelTrait::default()
    }

    fn before_save(self, _insert: bool) -> Result<Self, DbErr> {
        Ok(self)
    }

    // fn after_save(
    //     model: EntityTrait::Model,
    //     insert: bool,
    // ) -> Result<EntityTrait::Model, DbErr> {
    //     Ok(model)
    // }

    fn before_delete(self) -> Result<Self, DbErr> {
        Ok(self)
    }

    fn after_delete(self) -> Result<Self, DbErr> {
        Ok(self)
    }
}