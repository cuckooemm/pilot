use crate::model::app::Model;
use crate::{prelude::db_cli, AppActive, AppColumn, AppEntity, ID};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};

pub async fn insert_one(app: AppActive) -> Result<Model, DbErr> {
    app.insert(db_cli()).await
}

pub async fn find_all() -> Result<Vec<Model>, DbErr> {
    AppEntity::find().all(db_cli()).await
}

pub async fn is_exist(app_id: &String) -> Result<Option<ID>, DbErr> {
    // 查找 app_id 是否存在
    AppEntity::find()
        .select_only()
        .column(AppColumn::Id)
        .filter(AppColumn::AppId.eq(app_id.clone()))
        .into_model::<ID>()
        .one(db_cli())
        .await
}
