use super::master;

use entity::orm::{ActiveModelTrait, DbErr, EntityTrait};
use entity::{PublicationRecordActive, PublicationRecordEntity, PublicationRecordModel};

pub async fn insert_one(app: PublicationRecordActive) -> Result<PublicationRecordModel, DbErr> {
    app.insert(master()).await
}

pub async fn find_all() -> Result<Vec<PublicationRecordModel>, DbErr> {
    PublicationRecordEntity::find().all(master()).await
}
