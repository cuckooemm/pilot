use crate::model::app_extend::Model;
use crate::{prelude::db_cli, AppExtendActive, AppExtendColumn, AppExtendEntity, ID};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};

pub async fn insert_one(app: AppExtendActive) -> Result<Model, DbErr> {
    app.insert(db_cli()).await
}

pub async fn find_all() -> Result<Vec<Model>, DbErr> {
    AppExtendEntity::find().all(db_cli()).await
}


pub async fn find_by_app_all(app_id: Option<String>) -> Result<Vec<Model>, DbErr> {
    let mut stmt = AppExtendEntity::find();
    if let Some(app_id) = app_id {
        stmt = stmt.filter(AppExtendColumn::AppId.eq(app_id))
    }
    stmt.all(db_cli()).await
}

pub async fn is_exist(app_id: &String,name: &String) -> Result<Option<ID>, DbErr> {
    // 查找 app_id 是否存在
    AppExtendEntity::find()
        .select_only()
        .column(AppExtendColumn::Id)
        .filter(AppExtendColumn::AppId.eq(app_id.clone()))
        .filter(AppExtendColumn::Name.eq(name.clone()))
        .into_model::<ID>()
        .one(db_cli())
        .await
}
