use super::Conn;

use entity::common::enums::Status;
use entity::model::{
    cluster::ClusterItem, ClusterActive, ClusterColumn, ClusterEntity, ClusterModel,
};
use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{SecretData, ID};

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
    pub async fn find_app_cluster(
        &self,
        app_id: String,
        cluster: String,
    ) -> Result<Option<u64>, DbErr> {
        let r = ClusterEntity::find()
            .select_only()
            .column(ClusterColumn::Id)
            .filter(ClusterColumn::App.eq(app_id))
            .filter(ClusterColumn::Cluster.eq(cluster))
            .into_model::<ID>()
            .one(Conn::conn().main())
            .await?;
        Ok(r.and_then(|r| Some(r.id)))
    }

    pub async fn update_by_id(&self, model: ClusterActive, id: u64) -> Result<(), DbErr> {
        ClusterEntity::update_many()
            .set(model)
            .filter(ClusterColumn::Id.eq(id))
            .exec(Conn::conn().main())
            .await?;
        Ok(())
    }

    pub async fn find_cluster_by_app(
        &self,
        app: String,
        status: Option<Status>,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<ClusterModel>, DbErr> {
        let mut stmt = ClusterEntity::find()
            .filter(ClusterColumn::App.eq(app))
            .offset(offset)
            .limit(limit);
        if let Some(status) = status {
            stmt = stmt.filter(ClusterColumn::Status.eq(status));
        }
        stmt.all(Conn::conn().slaver()).await
    }

    pub async fn get_secret_by_cluster(
        &self,
        app: String,
        cluster: String,
    ) -> Result<Option<SecretData>, DbErr> {
        ClusterEntity::find()
            .select_only()
            .column(ClusterColumn::Secret)
            .filter(ClusterColumn::App.eq(app))
            .filter(ClusterColumn::Cluster.eq(cluster))
            .into_model::<SecretData>()
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
