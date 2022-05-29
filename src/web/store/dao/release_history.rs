use super::{master, slaver};

use entity::orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::release_history::{HistoryItem, HistoryNamespaceID};
use entity::{ReleaseHistoryActive, ReleaseHistoryColumn, ReleaseHistoryEntity};

pub async fn add(active: ReleaseHistoryActive) -> Result<u64, DbErr> {
    let r = ReleaseHistoryEntity::insert(active).exec(master()).await?;
    Ok(r.last_insert_id)
}

pub async fn get_namespace_id(id: u64) -> Result<Option<u64>, DbErr> {
    let entity = ReleaseHistoryEntity::find_by_id(id)
        .select_only()
        .column(ReleaseHistoryColumn::NamespaceId)
        .into_model::<HistoryNamespaceID>()
        .one(master())
        .await?;
    Ok(entity.and_then(|x| Some(x.namespace_id)))
}
pub async fn get_namespace_history(
    namespace_id: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<HistoryItem>, DbErr> {
    ReleaseHistoryEntity::find()
        .offset(offset)
        .limit(limit)
        .select_only()
        .column(ReleaseHistoryColumn::Id)
        .column(ReleaseHistoryColumn::ReleaseId)
        .column(ReleaseHistoryColumn::Change)
        .filter(ReleaseHistoryColumn::NamespaceId.eq(namespace_id))
        .filter(ReleaseHistoryColumn::DeletedAt.eq(0_u64))
        .into_model::<HistoryItem>()
        .all(slaver())
        .await
}
