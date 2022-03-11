use super::master;

use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{ClusterActive, ClusterColumn, ClusterEntity, ClusterModel, SecretData, ID};

pub async fn insert_one(app: ClusterActive) -> Result<ClusterModel, DbErr> {
    app.insert(master()).await
}

pub async fn find_all() -> Result<Vec<ClusterModel>, DbErr> {
    ClusterEntity::find().all(master()).await
}

pub async fn find_by_app_all(app_id: Option<String>) -> Result<Vec<ClusterModel>, DbErr> {
    let mut stmt = ClusterEntity::find();
    if let Some(app_id) = app_id {
        stmt = stmt.filter(ClusterColumn::AppId.eq(app_id))
    }
    stmt.all(master()).await
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
        .one(master())
        .await
}

pub async fn is_exist(app_id: &String, cluster: &String) -> Result<Option<ID>, DbErr> {
    ClusterEntity::find()
        .select_only()
        .column(ClusterColumn::Id)
        .filter(ClusterColumn::AppId.eq(app_id.clone()))
        .filter(ClusterColumn::Name.eq(cluster.clone()))
        .into_model::<ID>()
        .one(master())
        .await
}
