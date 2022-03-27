use super::{master, publication_record, slaver};

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
    let item = ItemEntity::find_by_id(item_id).one(master()).await?;
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
    let transaction = master()
        .transaction::<_, (), DbErr>(|tx| {
            Box::pin(async move {
                let mut entity: ItemActive = item.clone().into();
                // 更新记录 status 字段至 Publication, 如果成功继续下一步，如果已被更新则返回
                entity.status = Set(Status::Publication);
                // 更新数据库
                let result = ItemEntity::update_many()
                    .set(entity)
                    .filter(ItemColumn::Id.eq(item_id))
                    .filter(ItemColumn::Version.eq(version))
                    .exec(tx)
                    .await?;
                if result.rows_affected == 0 {
                    return Ok(());
                }
                // 写入或更新 Publication表 和 Publication_record 表
                let publication_id = PublicationEntity::find()
                    .select_only()
                    .column(PublicationColumn::Id)
                    .filter(PublicationColumn::ItemId.eq(item_id))
                    .into_model::<ID>()
                    .one(tx)
                    .await?;
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
                match publication_id {
                    Some(id) => {
                        PublicationEntity::update_many()
                            .set(publication)
                            .filter(PublicationColumn::Id.eq(id.id))
                            .exec(tx)
                            .await?;
                    }
                    None => {
                        PublicationEntity::insert(publication).exec(tx).await?;
                    }
                };

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

pub async fn rollback_item(record_id: i64, remark: String) -> Result<(), DbErr> {
    let record = publication_record::find_by_id(record_id).await?;
    if record.is_none() {
        return Err(DbErr::RecordNotFound(format!(
            "Not found record data by {}",
            record_id
        )));
    }
    let record = record.unwrap();
    let item = ItemEntity::find_by_id(record.item_id).one(master()).await?;
    if item.is_none() {
        return Err(DbErr::RecordNotFound(format!(
            "Not found record item data by {}",
            record_id
        )));
    }
    let item = item.unwrap();
    let published = PublicationEntity::find()
        .filter(PublicationColumn::ItemId.eq(item.id))
        .one(master())
        .await?;
    if published.is_none() {
        return Err(DbErr::RecordNotFound(format!(
            "Not found record item data by {}",
            record_id
        )));
    }
    let published = published.unwrap();
    // 如果回退版本与已发布版本相等 直接返回
    if published.version == record.version {
        return Ok(());
    }
    let transaction = master()
        .transaction::<_, (), DbErr>(|tx| {
            Box::pin(async move {
                // 如果item 状态为已发布  且 回退版本不等于当前item版本 置状态为未发布
                if item.status == Status::Publication && record.version != item.version {
                    let mut entity: ItemActive = item.clone().into();
                    entity.status = Set(Status::Normal);
                    ItemEntity::update_many()
                        .set(entity)
                        .filter(ItemColumn::Id.eq(item.id))
                        .exec(tx)
                        .await?;
                }
                // 如果回退版本等于当前item版本且状态为未发布置状态为已发布
                if record.version == item.version && item.status != Status::Publication {
                    let mut entity: ItemActive = item.clone().into();
                    entity.status = Set(Status::Publication);
                    ItemEntity::update_many()
                        .set(entity)
                        .filter(ItemColumn::Id.eq(item.id))
                        .exec(tx)
                        .await?;
                }
                // 将记录写入 publiction
                let publication = PublicationActive {
                    key: Set(record.key.clone()),
                    value: Set(record.value.clone()),
                    category: Set(record.category.clone()),
                    remark: Set(remark.clone()),
                    publish_user_id: NotSet, // TODO 发布者id
                    version: Set(record.version),
                    ..Default::default()
                };
                PublicationEntity::update_many()
                    .set(publication)
                    .filter(PublicationColumn::Id.eq(published.id))
                    .exec(tx)
                    .await?;
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
