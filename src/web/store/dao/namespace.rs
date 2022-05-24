use super::{master, slaver};

use entity::namespace::NamespaceItem;
use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{NamespaceActive, NamespaceColumn, NamespaceEntity, NamespaceModel, Scope, ID};

pub async fn add(namespace: NamespaceActive) -> Result<u64, DbErr> {
    let r = NamespaceEntity::insert(namespace).exec(master()).await?;
    Ok(r.last_insert_id)
}

pub async fn find_all() -> Result<Vec<NamespaceModel>, DbErr> {
    NamespaceEntity::find().all(master()).await
}

pub async fn get_namespace_by_appcluster(
    app_id: String,
    cluster: String,
) -> Result<Vec<NamespaceItem>, DbErr> {
    NamespaceEntity::find()
        .select_only()
        .column(NamespaceColumn::Id)
        .column(NamespaceColumn::Namespace)
        .filter(NamespaceColumn::AppId.eq(app_id))
        .filter(NamespaceColumn::Cluster.eq(cluster))
        .filter(NamespaceColumn::DeletedAt.eq(0_u64))
        .filter(NamespaceColumn::Scope.eq(Scope::Private))
        .into_model::<NamespaceItem>()
        .all(slaver())
        .await
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
        .filter(NamespaceColumn::Namespace.is_in(namespace.clone()))
        .into_model::<ID>()
        .all(master())
        .await
}

pub async fn is_exist_by_id(id: u64) -> Result<bool, DbErr> {
    let entity = NamespaceEntity::find_by_id(id)
        .select_only()
        .column(NamespaceColumn::Id)
        .into_model::<ID>()
        .one(master())
        .await?;
    Ok(entity.is_some())
}

pub async fn get_app_info(id: u64) -> Result<Option<NamespaceModel>, DbErr> {
    NamespaceEntity::find_by_id(id).one(master()).await
}

pub async fn is_exist(app_id: String, cluster: String, namespace: String) -> Result<bool, DbErr> {
    let entity = NamespaceEntity::find()
        .select_only()
        .column(NamespaceColumn::Id)
        .filter(NamespaceColumn::AppId.eq(app_id))
        .filter(NamespaceColumn::Cluster.eq(cluster))
        .filter(NamespaceColumn::Namespace.eq(namespace))
        .filter(NamespaceColumn::DeletedAt.eq(0_u64))
        .into_model::<ID>()
        .one(master())
        .await?;
    Ok(entity.is_some())
}

pub async fn get_namespace_id(
    app_id: String,
    cluster: String,
    namespace: String,
) -> Result<Option<u64>, DbErr> {
    let entity = NamespaceEntity::find()
        .select_only()
        .column(NamespaceColumn::Id)
        .filter(NamespaceColumn::AppId.eq(app_id))
        .filter(NamespaceColumn::Cluster.eq(cluster))
        .filter(NamespaceColumn::Namespace.eq(namespace))
        .filter(NamespaceColumn::DeletedAt.eq(0_u64))
        .into_model::<ID>()
        .one(slaver())
        .await?;
    Ok(entity.and_then(|x| Some(x.id)))
}
