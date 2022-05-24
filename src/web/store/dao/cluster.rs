use super::{master, slaver};

use entity::cluster::ClusterItem;
use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{ClusterActive, ClusterColumn, ClusterEntity, ClusterModel, SecretData, ID};

pub async fn add(cluster: ClusterActive) -> Result<u64, DbErr> {
    let r = ClusterEntity::insert(cluster).exec(master()).await?;
    Ok(r.last_insert_id)
}

pub async fn find_all() -> Result<Vec<ClusterModel>, DbErr> {
    ClusterEntity::find().all(master()).await
}

pub async fn find_app_cluster(
    app_id: String,
    cluster: String,
) -> Result<Option<ClusterModel>, DbErr> {
    ClusterEntity::find()
        .select_only()
        .column(ClusterColumn::Id)
        .filter(ClusterColumn::AppId.eq(app_id))
        .filter(ClusterColumn::Name.eq(cluster))
        .filter(ClusterColumn::DeletedAt.eq(0_u64))
        .one(master())
        .await
}

pub async fn update_by_id(model: ClusterActive, id: u64) -> Result<(), DbErr> {
    let x = ClusterEntity::update_many()
        .set(model)
        .filter(ClusterColumn::Id.eq(id))
        .exec(master())
        .await?;
    Ok(())
}

pub async fn find_cluster_by_app(app_id: String) -> Result<Vec<ClusterItem>, DbErr> {
    ClusterEntity::find()
        .select_only()
        .column(ClusterColumn::Id)
        .column(ClusterColumn::Name)
        .filter(ClusterColumn::AppId.eq(app_id))
        .filter(ClusterColumn::DeletedAt.eq(0_u64))
        .into_model::<ClusterItem>()
        .all(slaver())
        .await
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
        .one(slaver())
        .await
}

pub async fn is_exist(app_id: String, cluster: String) -> Result<bool, DbErr> {
    let entity = ClusterEntity::find()
        .select_only()
        .column(ClusterColumn::Id)
        .filter(ClusterColumn::AppId.eq(app_id))
        .filter(ClusterColumn::Name.eq(cluster))
        .filter(ClusterColumn::DeletedAt.eq(0_u64))
        .into_model::<ID>()
        .one(master())
        .await?;
    Ok(entity.is_some())
}
