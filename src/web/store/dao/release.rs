use super::Conn;

use entity::model::{
    ReleaseActive, ReleaseColumn, ReleaseEntity, ReleaseHistoryActive, ReleaseHistoryEntity,
    ReleaseHistoryModel,
};
use entity::model::{ReleaseHistoryColumn, ReleaseModel};
use entity::orm::{
    ColumnTrait, DbErr, EntityTrait, NotSet, QueryFilter, QueryOrder, QuerySelect, Set,
    TransactionError, TransactionTrait,
};

#[derive(Debug, Clone, Default)]
pub struct Release;

impl Release {
    pub async fn publication(
        &self,
        namespace_id: u64,
        name: String,
        remark: String,
        version: u64,
        configurations: String,
        mut change: Vec<ReleaseHistoryActive>,
    ) -> Result<bool, DbErr> {
        let release = ReleaseActive {
            namespace_id: Set(namespace_id),
            name: Set(name),
            remark: Set(remark),
            configurations: Set(configurations),
            ..Default::default()
        };
        let transaction = Conn::conn()
            .main()
            .transaction::<_, bool, DbErr>(|tx| {
                Box::pin(async move {
                    let latest_version = ReleaseEntity::find()
                        .select_only()
                        .column(ReleaseColumn::Version)
                        .filter(ReleaseColumn::NamespaceId.eq(namespace_id))
                        .order_by_desc(ReleaseColumn::Version)
                        .lock_exclusive()
                        .into_tuple::<u64>()
                        .one(tx)
                        .await?
                        .unwrap_or_default();
                    if version != latest_version {
                        return Ok(false);
                    }
                    let release_version = ReleaseEntity::insert(release)
                        .exec(tx)
                        .await?
                        .last_insert_id;
                    for c in change.iter_mut() {
                        c.release_version = Set(release_version);
                    }
                    ReleaseHistoryEntity::insert_many(change).exec(tx).await?;
                    Ok(true)
                })
            })
            .await
            .map_err(|e| match e {
                TransactionError::Connection(err) => {
                    return err;
                }
                TransactionError::Transaction(err) => {
                    return DbErr::Custom(err.to_string());
                }
            })?;
        Ok(transaction)
    }
    pub async fn list_item(
        &self,
        id: u64,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<ReleaseHistoryModel>, DbErr> {
        ReleaseHistoryEntity::find()
            .filter(ReleaseHistoryColumn::ItemId.eq(id))
            .offset(offset)
            .limit(limit)
            .all(Conn::conn().slaver())
            .await
    }
    pub async fn get_namespace_last_release(
        &self,
        namespace_id: u64,
    ) -> Result<Option<(u64, String)>, DbErr> {
        ReleaseEntity::find()
            .select_only()
            .columns([ReleaseColumn::Version, ReleaseColumn::Configurations])
            .filter(ReleaseColumn::NamespaceId.eq(namespace_id))
            .order_by_desc(ReleaseColumn::Version)
            .into_tuple()
            .one(Conn::conn().slaver())
            .await
    }

    pub async fn addition_history(&self, active: ReleaseHistoryActive) -> Result<u64, DbErr> {
        let r = ReleaseHistoryEntity::insert(active)
            .exec(Conn::conn().main())
            .await?;
        Ok(r.last_insert_id)
    }
    pub async fn get_release_by_id(&self, id: u64) -> Result<Option<(u64, String)>, DbErr> {
        ReleaseEntity::find_by_id(id)
            .columns([ReleaseColumn::NamespaceId, ReleaseColumn::Configurations])
            .into_tuple()
            .one(Conn::conn().slaver())
            .await
    }

    pub async fn get_namespace_release_list(
        &self,
        namespace_id: u64,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<ReleaseModel>, DbErr> {
        ReleaseEntity::find()
            .filter(ReleaseColumn::NamespaceId.eq(namespace_id))
            .offset(offset)
            .limit(limit)
            .all(Conn::conn().slaver())
            .await
    }
}
