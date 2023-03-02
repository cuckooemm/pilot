use super::Conn;

use entity::orm::{
    ColumnTrait, DbErr, EntityTrait, NotSet, QueryFilter, QueryOrder, QuerySelect, Set,
    TransactionError, TransactionTrait,
};
use entity::{
    model::{
        item::ItemDesc,
        release::{Effective, ReleaseConfig},
        release_history::{HistoryItem, HistoryNamespaceID},
        ReleaseActive, ReleaseColumn, ReleaseEntity, ReleaseHistoryActive, ReleaseHistoryColumn,
        ReleaseHistoryEntity,
    },
    ID,
};

#[derive(Debug, Clone, Default)]
pub struct Release;

impl Release {
    pub async fn publication_item(
        &self,
        r_id: u64,
        name: String,
        namespace_id: u64,
        remark: String,
        config: Vec<ItemDesc>,
        change: Vec<ItemDesc>,
        user_id: u32,
    ) -> Result<bool, DbErr> {
        // 序列化
        let config_data = serde_json::to_string(&config).unwrap();
        let change_data = serde_json::to_string(&change).unwrap();
        let transaction = Conn::conn()
            .main()
            .transaction::<_, bool, DbErr>(|tx| {
                Box::pin(async move {
                    let id = ReleaseEntity::find()
                        .select_only()
                        .column(ReleaseColumn::Id)
                        .filter(ReleaseColumn::NamespaceId.eq(namespace_id))
                        .order_by_desc(ReleaseColumn::Id)
                        .lock_exclusive()
                        .into_model::<ID>()
                        .one(tx)
                        .await?
                        .and_then(|x| Some(x.id))
                        .unwrap_or_default();
                    if r_id != id {
                        // 已被发布过
                        return Ok(false);
                    }

                    let release = ReleaseActive {
                        namespace_id: Set(namespace_id),
                        name: Set(name),
                        configurations: Set(config_data),
                        remark: Set(remark),
                        is_abandoned: Set(Effective::Valid),
                        ..Default::default()
                    };
                    let id = ReleaseEntity::insert(release).exec(tx).await?;
                    // 增加更改记录
                    let history = ReleaseHistoryActive {
                        namespace_id: Set(namespace_id),
                        change: Set(change_data),
                        release_id: Set(id.last_insert_id),
                        ..Default::default()
                    };
                    ReleaseHistoryEntity::insert(history).exec(tx).await?;
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

    pub async fn rollback_item(&self, id: u64, remark: String) -> Result<(), DbErr> {
        let history = ReleaseHistoryEntity::find_by_id(id)
            .one(Conn::conn().main())
            .await?;
        if history.is_none() {
            return Err(DbErr::RecordNotFound(
                "Not found release history".to_owned(),
            ));
        }
        let history = history.unwrap();
        let release = ReleaseEntity::find_by_id(history.release_id)
            .one(Conn::conn().main())
            .await?;
        if release.is_none() {
            return Err(DbErr::RecordNotFound(
                "Not found release history".to_owned(),
            ));
        }
        let release = release.unwrap();
        let mut release: ReleaseActive = release.into();
        release.updated_at = NotSet;
        release.created_at = NotSet;
        release.id = NotSet;
        release.is_abandoned = NotSet;
        release.remark = Set(remark);

        let r = ReleaseEntity::insert(release)
            .exec(Conn::conn().main())
            .await?;
        let mut history: ReleaseHistoryActive = history.into();
        history.created_at = NotSet;
        history.updated_at = NotSet;
        history.id = NotSet;
        history.release_id = Set(r.last_insert_id);
        ReleaseHistoryEntity::insert(history)
            .exec(Conn::conn().main())
            .await?;
        Ok(())
    }

    pub async fn get_namespace_config(
        &self,
        namespace_id: u64,
    ) -> Result<Option<ReleaseConfig>, DbErr> {
        ReleaseEntity::find()
            .select_only()
            .column(ReleaseColumn::Id)
            .column(ReleaseColumn::Configurations)
            .filter(ReleaseColumn::NamespaceId.eq(namespace_id))
            .order_by_desc(ReleaseColumn::Id)
            .into_model::<ReleaseConfig>()
            .one(Conn::conn().slaver())
            .await
    }

    pub async fn addition_history(&self, active: ReleaseHistoryActive) -> Result<u64, DbErr> {
        let r = ReleaseHistoryEntity::insert(active)
            .exec(Conn::conn().main())
            .await?;
        Ok(r.last_insert_id)
    }
    pub async fn get_history_namespace_id(&self, id: u64) -> Result<Option<u64>, DbErr> {
        let entity = ReleaseHistoryEntity::find_by_id(id)
            .select_only()
            .column(ReleaseHistoryColumn::NamespaceId)
            .into_model::<HistoryNamespaceID>()
            .one(Conn::conn().main())
            .await?;
        Ok(entity.and_then(|x| Some(x.namespace_id)))
    }
    pub async fn get_namespace_history(
        &self,
        namespace_id: u64,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<HistoryItem>, DbErr> {
        ReleaseHistoryEntity::find()
            .offset(offset)
            .limit(limit)
            .select_only()
            .column(ReleaseHistoryColumn::Id)
            .column(ReleaseHistoryColumn::ReleaseId)
            .column(ReleaseHistoryColumn::Change)
            .filter(ReleaseHistoryColumn::NamespaceId.eq(namespace_id))
            .into_model::<HistoryItem>()
            .all(Conn::conn().slaver())
            .await
    }
}
