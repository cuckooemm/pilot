use super::master;

use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{AppExtendActive, AppExtendColumn, AppExtendEntity, AppExtendModel, ID};

pub async fn insert_one(app: AppExtendActive) -> Result<AppExtendModel, DbErr> {
    app.insert(master()).await
}

pub async fn find_all() -> Result<Vec<AppExtendModel>, DbErr> {
    AppExtendEntity::find().all(master()).await
}

pub async fn find_by_app_all(app_id: Option<String>) -> Result<Vec<AppExtendModel>, DbErr> {
    let mut stmt = AppExtendEntity::find();
    if let Some(app_id) = app_id {
        stmt = stmt.filter(AppExtendColumn::AppId.eq(app_id))
    }
    stmt.all(master()).await
}

pub async fn is_exist(app_id: &String, name: &String) -> Result<Option<ID>, DbErr> {
    // 查找 app_id 是否存在
    AppExtendEntity::find()
        .select_only()
        .column(AppExtendColumn::Id)
        .filter(AppExtendColumn::AppId.eq(app_id.clone()))
        .into_model::<ID>()
        .one(master())
        .await
}
