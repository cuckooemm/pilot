use super::{master, slaver};

use entity::item::ItemDesc;
use entity::orm::{
    ColumnTrait, DbErr, EntityTrait, NotSet, QueryFilter, QueryOrder, QuerySelect, Set,
    TransactionError, TransactionTrait,
};
use entity::release::{Effective, ReleaseConfig};
use entity::{
    ReleaseActive, ReleaseColumn, ReleaseEntity, ReleaseHistoryActive, ReleaseHistoryEntity, ID,
};

pub async fn publication_item(
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
    let transaction = master()
        .transaction::<_, bool, DbErr>(|tx| {
            Box::pin(async move {
                let id = ReleaseEntity::find()
                    .select_only()
                    .column(ReleaseColumn::Id)
                    .filter(ReleaseColumn::NamespaceId.eq(namespace_id))
                    .filter(ReleaseColumn::DeletedAt.eq(0_u64))
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
                    publish_user_id: Set(user_id),
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
        .await;
    if let Err(e) = transaction {
        match e {
            TransactionError::Connection(err) => {
                return Err(err);
            }
            TransactionError::Transaction(err) => {
                return Err(DbErr::Exec(err.to_string()));
            }
        }
    }
    Ok(transaction.unwrap())
}

pub async fn rollback_item(id: u64, remark: String) -> Result<(), DbErr> {
    let history = ReleaseHistoryEntity::find_by_id(id).one(master()).await?;
    if history.is_none() {
        return Err(DbErr::RecordNotFound(
            "Not found release history".to_owned(),
        ));
    }
    let history = history.unwrap();
    let release = ReleaseEntity::find_by_id(history.release_id)
        .one(master())
        .await?;
    if release.is_none() {
        return Err(DbErr::RecordNotFound(
            "Not found release history".to_owned(),
        ));
    }
    let release = release.unwrap();
    let mut release: ReleaseActive = release.into();
    release.deleted_at = NotSet;
    release.updated_at = NotSet;
    release.created_at = NotSet;
    release.id = NotSet;
    release.is_abandoned = NotSet;
    release.remark = Set(remark);

    let r = ReleaseEntity::insert(release).exec(master()).await?;
    let mut history: ReleaseHistoryActive = history.into();
    history.created_at = NotSet;
    history.updated_at = NotSet;
    history.deleted_at = NotSet;
    history.id = NotSet;
    history.release_id = Set(r.last_insert_id);
    ReleaseHistoryEntity::insert(history).exec(master()).await?;
    Ok(())
}

pub async fn get_namespace_config(namespace_id: u64) -> Result<Option<ReleaseConfig>, DbErr> {
    ReleaseEntity::find()
        .select_only()
        .column(ReleaseColumn::Id)
        .column(ReleaseColumn::Configurations)
        .filter(ReleaseColumn::NamespaceId.eq(namespace_id))
        .filter(ReleaseColumn::DeletedAt.eq(0_u64))
        .order_by_desc(ReleaseColumn::Id)
        .into_model::<ReleaseConfig>()
        .one(slaver())
        .await
}
