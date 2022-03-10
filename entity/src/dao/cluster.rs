use crate::model::cluster::Model;
use crate::{prelude::db_cli, ClusterActive, ClusterColumn, ClusterEntity};
use crate::{SecretData, ID};

use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};

pub async fn insert_one(app: ClusterActive) -> Result<Model, DbErr> {
    app.insert(db_cli()).await
}

pub async fn find_all() -> Result<Vec<Model>, DbErr> {
    ClusterEntity::find().all(db_cli()).await
}

pub async fn find_by_app_all(app_id: Option<String>) -> Result<Vec<Model>, DbErr> {
    let mut stmt = ClusterEntity::find();
    if let Some(app_id) = app_id {
        stmt = stmt.filter(ClusterColumn::AppId.eq(app_id))
    }
    stmt.all(db_cli()).await
}

pub async fn get_secret_by_cluster(
    app_id: &String,
    cluster: &String,
) -> Result<Option<SecretData>, DbErr> {
    ClusterEntity::find()
        .select_only()
        .column(ClusterColumn::Secret)
        .filter(ClusterColumn::AppId.eq(app_id.clone()))
        .filter(ClusterColumn::Name.eq(cluster.clone()))
        .into_model::<SecretData>()
        .one(db_cli())
        .await
}

pub async fn is_exist(app_id: &String, cluster: &String) -> Result<Option<ID>, DbErr> {
    ClusterEntity::find()
        .select_only()
        .column(ClusterColumn::Id)
        .filter(ClusterColumn::AppId.eq(app_id.clone()))
        .filter(ClusterColumn::Name.eq(cluster.clone()))
        .into_model::<ID>()
        .one(db_cli())
        .await
}
