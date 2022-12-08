use super::Conn;

use entity::item::{ItemData, ItemDesc};
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect, Set,
};
use entity::{ItemActive, ItemCategory, ItemColumn, ItemEntity, ItemModel, ID};

#[derive(Debug, Clone, Default)]
pub struct Item;
impl Item {
    pub async fn addition(&self, app: ItemActive) -> Result<u64, DbErr> {
        let r = ItemEntity::insert(app).exec(Conn::conn().main()).await?;
        Ok(r.last_insert_id)
    }

    pub async fn get_item_by_ids(&self, ids: Vec<u64>) -> Result<Vec<ItemData>, DbErr> {
        ItemEntity::find()
            .select_only()
            .column(ItemColumn::Id)
            .column(ItemColumn::NamespaceId)
            .column(ItemColumn::Key)
            .column(ItemColumn::Value)
            .column(ItemColumn::Category)
            .column(ItemColumn::Version)
            .filter(ItemColumn::Id.is_in(ids))
            .into_model::<ItemData>()
            .all(Conn::conn().main())
            .await
    }

    pub async fn get_namespace_items(&self, id: u64) -> Result<Vec<ItemDesc>, DbErr> {
        ItemEntity::find()
            .select_only()
            .column(ItemColumn::Key)
            .column(ItemColumn::Value)
            .column(ItemColumn::Category)
            .column(ItemColumn::Version)
            .filter(ItemColumn::NamespaceId.eq(id))
            .into_model::<ItemDesc>()
            .all(Conn::conn().main())
            .await
    }

    pub async fn find_by_nsid_all(
        &self,
        ns_id: u64,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<ItemModel>, DbErr> {
        ItemEntity::find()
            .offset(offset)
            .limit(limit)
            .filter(ItemColumn::NamespaceId.eq(ns_id))
            .all(Conn::conn().slaver())
            .await
    }

    pub async fn is_key_exist(&self, ns_id: u64, key: String) -> Result<bool, DbErr> {
        let entity = ItemEntity::find()
            .select_only()
            .column(ItemColumn::Id)
            .filter(ItemColumn::NamespaceId.eq(ns_id))
            .filter(ItemColumn::Key.eq(key))
            .into_model::<ID>()
            .one(Conn::conn().main())
            .await?;
        Ok(entity.is_some())
    }

    pub async fn find_by_id(&self, id: u64) -> Result<Option<ItemModel>, DbErr> {
        ItemEntity::find_by_id(id).one(Conn::conn().main()).await
    }

    pub async fn update(
        &self,
        entity: ItemModel,
        key: Option<String>,
        value: Option<String>,
        category: Option<String>,
        remark: Option<String>,
        version: i64,
        modify_user_id: u32,
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
        // 无更新
        if !active.is_changed() {
            return Ok(false);
        }
        active.version = Set(entity.version + 1);

        // let result = active.save(master()).await?;
        let result = ItemEntity::update_many()
            .set(active)
            .filter(ItemColumn::Id.eq(entity.id))
            .filter(ItemColumn::Version.eq(version))
            .exec(Conn::conn().main())
            .await?;
        if result.rows_affected == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}
