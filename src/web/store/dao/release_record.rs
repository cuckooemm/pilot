use super::master;

use entity::orm::{ActiveModelTrait, DbErr, EntityTrait};
use entity::{ReleaseRecordActive, ReleaseRecordEntity, ReleaseRecordModel};

pub async fn insert_one(app: ReleaseRecordActive) -> Result<ReleaseRecordModel, DbErr> {
    app.insert(master()).await
}

pub async fn find_all() -> Result<Vec<ReleaseRecordModel>, DbErr> {
    ReleaseRecordEntity::find().all(master()).await
}
