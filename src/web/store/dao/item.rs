use super::master;

use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{ItemActive, ItemColumn, ItemEntity, ItemModel, ID};

pub async fn insert_one(app: ItemActive) -> Result<ItemModel, DbErr> {
    app.insert(master()).await
}

pub async fn find_all() -> Result<Vec<ItemModel>, DbErr> {
    ItemEntity::find().all(master()).await
}

pub async fn find_by_id_all(ns_id: i64) -> Result<Vec<ItemModel>, DbErr> {
    ItemEntity::find()
        .filter(ItemColumn::NamespaceId.eq(ns_id))
        .all(master())
        .await
}

pub async fn is_exist_key(ns_id: i64, key: &String) -> Result<Option<ID>, DbErr> {
    ItemEntity::find()
        .select_only()
        .column(ItemColumn::Id)
        .filter(ItemColumn::NamespaceId.eq(ns_id))
        .filter(ItemColumn::Key.eq(key.clone()))
        .into_model::<ID>()
        .one(master())
        .await
}
