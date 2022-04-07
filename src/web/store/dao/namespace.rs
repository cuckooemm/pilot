use super::master;

use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{NamespaceActive, NamespaceColumn, NamespaceEntity, NamespaceModel, ID};

pub async fn insert(namespace: NamespaceActive) -> Result<u64, DbErr> {
    let r = NamespaceEntity::insert(namespace).exec(master()).await?;
    Ok(r.last_insert_id)
}

pub async fn find_all() -> Result<Vec<NamespaceModel>, DbErr> {
    NamespaceEntity::find().all(master()).await
}

pub async fn find_by_app_cluster_all(
    app_id: Option<String>,
    cluster: Option<String>,
    offset: u64,
    limit: u64,
) -> Result<Vec<NamespaceModel>, DbErr> {
    let mut stmt = NamespaceEntity::find().offset(offset).limit(limit);
    if let Some(app_id) = app_id {
        stmt = stmt.filter(NamespaceColumn::AppId.eq(app_id.to_string()))
    }
    if let Some(cluster) = cluster {
        stmt = stmt.filter(NamespaceColumn::ClusterName.eq(cluster))
    }
    stmt.all(master()).await
}

pub async fn find_namespaceid_by_app_cluster(
    app_id: &String,
    cluster: &String,
    namespace: &Vec<&str>,
) -> Result<Vec<ID>, DbErr> {
    NamespaceEntity::find()
        .select_only()
        .column(NamespaceColumn::Id)
        .filter(NamespaceColumn::AppId.eq(app_id.clone()))
        .filter(NamespaceColumn::ClusterName.eq(cluster.clone()))
        .filter(NamespaceColumn::Namespace.is_in(namespace.clone()))
        .into_model::<ID>()
        .all(master())
        .await
}

pub async fn is_exist_by_id(id: u64) -> Result<Option<ID>, DbErr> {
    NamespaceEntity::find_by_id(id)
        .select_only()
        .column(NamespaceColumn::Id)
        .into_model::<ID>()
        .one(master())
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
        .one(master())
        .await
}
