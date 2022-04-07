use super::master;

use entity::orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{AppActive, AppColumn, AppEntity, AppModel, ID};

pub async fn insert(app: AppActive) -> Result<u64, DbErr> {
    let r = AppEntity::insert(app).exec(master()).await?;
    Ok(r.last_insert_id)
}

pub async fn find_all(offset: u64,limit: u64) -> Result<Vec<AppModel>,DbErr> {
    AppEntity::find()
    .offset(offset)
    .limit(limit)
    .all(master()).await

}
pub async fn is_exist(app_id: String) -> Result<Option<ID>, DbErr> {
    // 查找 app_id 是否存在
    AppEntity::find()
        .select_only()
        .column(AppColumn::Id)
        .filter(AppColumn::AppId.eq(app_id.clone()))
        .into_model::<ID>()
        .one(master())
        .await
}
