use super::master;

use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{ReleaseActive, ReleaseColumn, ReleaseEntity, ReleaseItem, ReleaseModel, ID};

pub async fn insert_one(app: ReleaseActive) -> Result<ReleaseModel, DbErr> {
    app.insert(master()).await
}

pub async fn find_all() -> Result<Vec<ReleaseModel>, DbErr> {
    ReleaseEntity::find().all(master()).await
}

pub async fn find_by_id_all(ns_id: i64) -> Result<Vec<ReleaseModel>, DbErr> {
    ReleaseEntity::find()
        .filter(ReleaseColumn::NamespaceId.eq(ns_id))
        .all(master())
        .await
}

pub async fn is_exist_key(ns_id: i64, key: &String) -> Result<Option<ID>, DbErr> {
    ReleaseEntity::find()
        .select_only()
        .column(ReleaseColumn::Id)
        .filter(ReleaseColumn::NamespaceId.eq(ns_id))
        .filter(ReleaseColumn::Key.eq(key.clone()))
        .into_model::<ID>()
        .one(master())
        .await
}

pub async fn get_val_by_namespace(namespace_id: i64) -> Result<Vec<ReleaseItem>, DbErr> {
    ReleaseEntity::find()
        .select_only()
        .column(ReleaseColumn::Key)
        .column(ReleaseColumn::Value)
        .column(ReleaseColumn::Category)
        .filter(ReleaseColumn::NamespaceId.eq(namespace_id))
        .into_model::<ReleaseItem>()
        .all(master())
        .await
}
