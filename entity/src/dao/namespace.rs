use crate::model::namespace::Model;
use crate::prelude::db_cli;
use crate::ID;
use crate::{NamespaceActive, NamespaceColumn, NamespaceEntity};

use sea_orm::{ActiveModelTrait, DbErr};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

pub async fn insert_one(app: NamespaceActive) -> Result<Model, DbErr> {
    app.insert(db_cli()).await
}

pub async fn find_all() -> Result<Vec<Model>, DbErr> {
    NamespaceEntity::find().all(db_cli()).await
}

pub async fn find_by_app_cluster_all(
    app_id: Option<String>,
    cluster: Option<String>,
) -> Result<Vec<Model>,DbErr> {
    let mut stmt = NamespaceEntity::find();
    if let Some(app_id) = app_id {
        stmt = stmt.filter(NamespaceColumn::AppId.eq(app_id.to_string()))
    }
    if let Some(cluster) = cluster {
        stmt = stmt.filter(NamespaceColumn::ClusterName.eq(cluster))
    }
    stmt.all(db_cli()).await
}
pub async fn get_namespace_id(id: i64, cluster: String) -> Result<(), DbErr> {
    // let c = ClusterEntity::find()
    // .select_only()
    // .column(ClusterColumn::Id)
    // .filter(ClusterColumn::AppId.eq(id))
    // .filter(ClusterColumn::Name.eq(cluster))
    // .into_model::<ID>()
    // .one(db)
    // .await?;
    Ok(())
}

pub async fn is_exist_by_id(id: i64) -> Result<Option<ID>,DbErr>{
    NamespaceEntity::find_by_id(id)
    .select_only()
    .column(NamespaceColumn::Id)
    .into_model::<ID>()
    .one(db_cli())
    .await
}

pub async fn is_exist(
    app_id: &String,
    cluster: &String,
    namespace: &String,
) -> Result<Option<ID>, DbErr> {
    NamespaceEntity::find()
        .select_only()
        .column(NamespaceColumn::Id)
        .filter(NamespaceColumn::AppId.eq(app_id.clone()))
        .filter(NamespaceColumn::ClusterName.eq(cluster.clone()))
        .filter(NamespaceColumn::Namespace.eq(namespace.clone()))
        .into_model::<ID>()
        .one(db_cli())
        .await
}
