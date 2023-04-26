use super::Conn;

use entity::common::enums::Status;
use entity::model::{ClusterActive, ClusterColumn, ClusterEntity, ClusterModel};
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};
use entity::response::model::ClusterItem;

#[derive(Debug, Clone, Default)]
pub struct Cluster;
impl Cluster {
    pub async fn addition(&self, cluster: ClusterActive) -> Result<ClusterModel, DbErr> {
        cluster.insert(Conn::conn().main()).await
    }
    pub async fn get_cluster_by_id(&self, id: u64) -> Result<Option<ClusterModel>, DbErr> {
        ClusterEntity::find_by_id(id).one(Conn::conn().main()).await
    }
    pub async fn update(&self, cluster: ClusterActive) -> Result<ClusterModel, DbErr> {
        cluster.update(Conn::conn().main()).await
    }

    pub async fn list_cluster_by_app(
        &self,
        app: String,
        status: Option<Status>,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<ClusterItem>, DbErr> {
        let mut stmt = ClusterEntity::find()
            .select_only()
            .columns([
                ClusterColumn::Id,
                ClusterColumn::Cluster,
                ClusterColumn::Describe,
                ClusterColumn::Status,
            ])
            .filter(ClusterColumn::App.eq(app))
            .offset(offset)
            .limit(limit);
        if let Some(status) = status {
            stmt = stmt.filter(ClusterColumn::Status.eq(status));
        }
        stmt.into_model::<ClusterItem>()
            .all(Conn::conn().slaver())
            .await
    }

    pub async fn count_cluster_by_app(
        &self,
        app: String,
        status: Option<Status>,
    ) -> Result<u64, DbErr> {
        let mut stmt = ClusterEntity::find()
            .select_only()
            .column_as(ClusterColumn::Id.count(), "count")
            .filter(ClusterColumn::App.eq(app));
        if let Some(status) = status {
            stmt = stmt.filter(ClusterColumn::Status.eq(status));
        }
        stmt.into_tuple::<u64>()
            .one(Conn::conn().slaver())
            .await
            .and_then(|c| Ok(c.unwrap_or_default()))
    }

    pub async fn get_secret_by_cluster(
        &self,
        app: String,
        cluster: String,
    ) -> Result<Option<String>, DbErr> {
        ClusterEntity::find()
            .select_only()
            .column(ClusterColumn::Secret)
            .filter(ClusterColumn::App.eq(app))
            .filter(ClusterColumn::Cluster.eq(cluster))
            .into_tuple::<String>()
            .one(Conn::conn().slaver())
            .await
    }

    pub async fn is_exist(&self, app: String, cluster: String) -> Result<bool, DbErr> {
        ClusterEntity::find()
            .select_only()
            .column(ClusterColumn::Id)
            .filter(ClusterColumn::App.eq(app))
            .filter(ClusterColumn::Cluster.eq(cluster))
            .into_tuple::<u64>()
            .one(Conn::conn().main())
            .await
            .and_then(|id| Ok(id.is_some()))
    }
}
