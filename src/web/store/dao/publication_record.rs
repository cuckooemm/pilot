use super::master;

use entity::orm::{ActiveModelTrait, DbErr, EntityTrait, QueryFilter, ColumnTrait, QueryOrder, PaginatorTrait, QuerySelect};
use entity::{PublicationRecordActive, PublicationRecordEntity, PublicationRecordModel,PublicationRecordColumn};

pub async fn insert(active: PublicationRecordActive) -> Result<i64, DbErr> {
    let r = PublicationRecordEntity::insert(active).exec(master()).await?;
    Ok(r.last_insert_id)
}

pub async fn find_all() -> Result<Vec<PublicationRecordModel>, DbErr> {
    PublicationRecordEntity::find().all(master()).await
}

pub async fn find_by_item(item_id: i64, offset: u64, limit: u64) ->  Result<Vec<PublicationRecordModel>, DbErr> {
    PublicationRecordEntity::find()
    .offset(offset)
    .limit(limit)
    .filter(PublicationRecordColumn::ItemId.eq(item_id))
    .order_by_desc(PublicationRecordColumn::Id)
    .all(master())
    .await
}
