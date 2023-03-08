use super::Conn;

use entity::common::enums::Status;
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect, Set,
};
use entity::{
    model::{
        item::{ItemData, ItemDesc},
        ItemActive, ItemColumn, ItemEntity, ItemModel,
    },
    ItemCategory, ID,
};

#[derive(Debug, Clone, Default)]
pub struct Item;
impl Item {
    pub async fn addition(&self, item: ItemActive) -> Result<ItemModel, DbErr> {
        item.insert(Conn::conn().main()).await
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

    pub async fn list_namespace_id(
        &self,
        namespace_id: u64,
        status: Option<Status>,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<ItemModel>, DbErr> {
        let mut stmt = ItemEntity::find()
            .offset(offset)
            .limit(limit)
            .filter(ItemColumn::NamespaceId.eq(namespace_id));
        if let Some(status) = status {
            stmt = stmt.filter(ItemColumn::Status.eq(status));
        };
        stmt.all(Conn::conn().slaver()).await
    }

    pub async fn is_key_exist(&self, namespace_id: u64, key: String) -> Result<bool, DbErr> {
        let entity = ItemEntity::find()
            .select_only()
            .column(ItemColumn::Id)
            .filter(ItemColumn::NamespaceId.eq(namespace_id))
            .filter(ItemColumn::Key.eq(key))
            .into_tuple::<u64>()
            .one(Conn::conn().main())
            .await?;
        Ok(entity.is_some())
    }

    pub async fn find_by_id(&self, id: u64) -> Result<Option<ItemModel>, DbErr> {
        ItemEntity::find_by_id(id).one(Conn::conn().main()).await
    }

    pub async fn update(&self, active: ItemActive, version: u64) -> Result<bool, DbErr> {
        let id = active.id.as_ref().clone();
        let result = ItemEntity::update_many()
            .set(active)
            .filter(ItemColumn::Id.eq(id))
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
