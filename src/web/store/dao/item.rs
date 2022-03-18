use super::master;

use entity::common::Status;
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect, Set,
};
use entity::{ItemActive, ItemCategory, ItemColumn, ItemEntity, ItemModel, ID};

pub async fn insert_one(app: ItemActive) -> Result<ItemModel, DbErr> {
    app.insert(master()).await
}

pub async fn find_all() -> Result<Vec<ItemModel>, DbErr> {
    ItemEntity::find().all(master()).await
}

pub async fn find_by_nsid_all(ns_id: i64) -> Result<Vec<ItemModel>, DbErr> {
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

pub async fn find_by_id(id: i64) -> Result<Option<ItemModel>, DbErr> {
    ItemEntity::find_by_id(id).one(master()).await
}

pub async fn update(
    entity: ItemModel,
    key: Option<String>,
    value: Option<String>,
    category: Option<String>,
    remark: Option<String>,
    version: i64,
) -> Result<bool, DbErr> {
    let mut active: ItemActive = entity.clone().into();

    if let Some(category) = category {
        let category: ItemCategory = category.into();
        if entity.category != category {
            active.category = Set(category);
        }
    }
    if let Some(key) = key {
        if entity.key != key {
            active.key = Set(key);
        }
    }
    if let Some(value) = value {
        if entity.value != value {
            active.value = Set(value);
        }
    }
    if let Some(remark) = remark {
        if entity.remark != remark {
            active.remark = Set(remark);
        }
    }
    active.status = Set(Status::Normal); // 状态重置
    active.version = Set(entity.version + 1);
    // active.modify_user_id = Set(0);
    // let result = active.save(master()).await?;
    let result = ItemEntity::update_many()
        .set(active)
        .filter(ItemColumn::Id.eq(entity.id))
        .filter(ItemColumn::Version.eq(version))
        .exec(master())
        .await?;
    if result.rows_affected == 0 {
        Ok(false)
    } else {
        Ok(true)
    }
}
