use super::{master, slaver};

use entity::common::Status;
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, NotSet, QueryFilter, QuerySelect, Set,
    TransactionError, TransactionTrait,
};
use entity::{
    ItemActive, ItemColumn, ItemEntity, ItemModel, PublicationActive, PublicationColumn,
    PublicationEntity, PublicationItem, PublicationModel, PublicationRecordActive,
    PublicationRecordEntity, ID,
};

pub async fn insert_one(app: PublicationActive) -> Result<PublicationModel, DbErr> {
    app.insert(master()).await
}

pub async fn find_all() -> Result<Vec<PublicationModel>, DbErr> {
    PublicationEntity::find().all(master()).await
}

pub async fn find_by_id_all(ns_id: i64) -> Result<Vec<PublicationModel>, DbErr> {
    PublicationEntity::find()
        .filter(PublicationColumn::NamespaceId.eq(ns_id))
        .all(master())
        .await
}

pub async fn is_exist_key(ns_id: i64, key: &String) -> Result<Option<ID>, DbErr> {
    PublicationEntity::find()
        .select_only()
        .column(PublicationColumn::Id)
        .filter(PublicationColumn::NamespaceId.eq(ns_id))
        .filter(PublicationColumn::Key.eq(key.clone()))
        .into_model::<ID>()
        .one(master())
        .await
}

pub async fn get_val_by_namespace(namespace_id: u64) -> Result<Vec<PublicationItem>, DbErr> {
    PublicationEntity::find()
        .select_only()
        .column(PublicationColumn::Key)
        .column(PublicationColumn::Value)
        .column(PublicationColumn::Category)
        .filter(PublicationColumn::NamespaceId.eq(namespace_id))
        .into_model::<PublicationItem>()
        .all(slaver())
        .await
}

pub async fn publication_item(item_id: i64, remark: String, version: i64) -> Result<(), DbErr> {
    let transaction = master()
        .transaction::<_, (), DbErr>(|tx| {
            Box::pin(async move {
                let item = ItemEntity::find_by_id(item_id)
                    .lock_exclusive() // 加锁记录 避免并发
                    .one(tx)
                    .await?;
                if item.is_none() {
                    return Err(DbErr::RecordNotFound(format!(
                        "Not found item data by {}",
                        item_id
                    )));
                }
                let item: ItemModel = item.unwrap();
                if item.version != version {
                    return Err(DbErr::Custom(format!("The item version is invalid")));
                }
                if item.status == Status::Publication {
                    // 已经发布过
                    return Ok(());
                }
                let mut entity: ItemActive = item.clone().into();
                // 更新记录 status 字段至 Publication, 如果成功继续下一步，如果已被更新则返回
                entity.status = Set(Status::Publication);
                // 更新数据库

                let item = entity.update(tx).await?;
                // 写入或更新 Publication表 和 Publication_record 表
                let publication = PublicationActive {
                    item_id: Set(item.id),
                    namespace_id: Set(item.namespace_id),
                    key: Set(item.key.clone()),
                    value: Set(item.value.clone()),
                    category: Set(item.category.clone()),
                    remark: Set(remark.clone()),
                    publish_user_id: NotSet, // TODO 发布者id
                    version: Set(item.version),
                    ..Default::default()
                };
                PublicationEntity::insert(publication).exec(tx).await?;

                let record = PublicationRecordActive {
                    item_id: Set(item.id),
                    namespace_id: Set(item.namespace_id),
                    key: Set(item.key),
                    value: Set(item.value),
                    category: Set(item.category),
                    remark: Set(remark),
                    publish_user_id: NotSet, // TODO 发布者id
                    version: Set(item.version),
                    ..Default::default()
                };
                PublicationRecordEntity::insert(record).exec(tx).await?;
                Ok(())
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
    Ok(())
}

pub async fn rollback_item(item_id: i64, version: i64) -> Result<(), DbErr> {
    // let transaction = master()
    // .transaction::<_,(),DbErr>(|tx| {
    //     Box::pin(async move {

    //     })
    // }).await;

    Ok(())
}
