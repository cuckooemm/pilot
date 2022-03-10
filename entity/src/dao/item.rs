use crate::model::item::Model;
use crate::ID;
use crate::{prelude::db_cli, ItemActive, ItemColumn, ItemEntity};

use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};

pub async fn insert_one(app: ItemActive) -> Result<Model, DbErr> {
    app.insert(db_cli()).await
}

pub async fn find_all() -> Result<Vec<Model>, DbErr> {
    ItemEntity::find().all(db_cli()).await
}

pub async fn find_by_id_all(ns_id: i64) -> Result<Vec<Model>, DbErr> {
    ItemEntity::find()
        .filter(ItemColumn::NamespaceId.eq(ns_id))
        .all(db_cli())
        .await
}

pub async fn is_exist_key(ns_id: i64, key: &String) -> Result<Option<ID>, DbErr> {
    ItemEntity::find()
        .select_only()
        .column(ItemColumn::Id)
        .filter(ItemColumn::NamespaceId.eq(ns_id))
        .filter(ItemColumn::Key.eq(key.clone()))
        .into_model::<ID>()
        .one(db_cli())
        .await
}
