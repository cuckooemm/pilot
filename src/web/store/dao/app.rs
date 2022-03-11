use super::master;

use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{AppActive, AppColumn, AppEntity, AppModel, ID};

pub async fn insert_one(app: AppActive) -> Result<AppModel, DbErr> {
    app.insert(master()).await
}

pub async fn find_all() -> Result<Vec<AppModel>, DbErr> {
    AppEntity::find().all(master()).await
}

pub async fn is_exist(app_id: &String) -> Result<Option<ID>, DbErr> {
    // 查找 app_id 是否存在
    AppEntity::find()
        .select_only()
        .column(AppColumn::Id)
        .filter(AppColumn::AppId.eq(app_id.clone()))
        .into_model::<ID>()
        .one(master())
        .await
}
