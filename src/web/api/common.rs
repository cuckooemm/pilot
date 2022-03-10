use entity::orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{AppColumn, AppEntity, ClusterColumn, ClusterEntity, ID};

pub async fn app_exist<'a, C>(db: &C, id: String) -> Result<Option<ID>, DbErr>
where
    C: ConnectionTrait,
{
    AppEntity::find()
        .select_only()
        .column(AppColumn::Id)
        .filter(AppColumn::AppId.eq(id))
        .into_model::<ID>()
        .one(db)
        .await
}

pub async fn app_cluster_exist<'a, C>(
    db: &C,
    id: String,
    namespace: String,
) -> Result<Option<ID>, DbErr>
where
    C: ConnectionTrait,
{
    ClusterEntity::find()
        .select_only()
        .column(ClusterColumn::Id)
        .filter(ClusterColumn::AppId.eq(id))
        .filter(ClusterColumn::Name.eq(namespace))
        .into_model::<ID>()
        .one(db)
        .await
}
