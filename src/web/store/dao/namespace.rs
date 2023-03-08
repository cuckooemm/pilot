use super::Conn;

use entity::common::enums::Status;
use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{
    model::{
        namespace::{NamespaceInfo},
        NamespaceActive, NamespaceColumn, NamespaceEntity, NamespaceModel,
    },
    Scope,
};

#[derive(Debug, Clone, Default)]
pub struct Namespace;

impl Namespace {
    pub async fn addition(&self, active: NamespaceActive) -> Result<NamespaceModel, DbErr> {
        active.insert(Conn::conn().main()).await
    }

    pub async fn update(&self, active: NamespaceActive) -> Result<NamespaceModel, DbErr> {
        active.update(Conn::conn().main()).await
    }

    pub async fn list_by_appcluster(
        &self,
        app: String,
        cluster: String,
        status: Option<Status>,
        scope: Option<Scope>,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<NamespaceModel>, DbErr> {
        let mut stmt = NamespaceEntity::find()
            .filter(NamespaceColumn::App.eq(app))
            .filter(NamespaceColumn::Cluster.eq(cluster))
            .offset(offset)
            .limit(limit);
        if let Some(status) = status {
            stmt = stmt.filter(NamespaceColumn::Status.eq(status));
        }
        if let Some(scope) = scope {
            stmt = stmt.filter(NamespaceColumn::Scope.eq(scope))
        }
        stmt.all(Conn::conn().slaver()).await
    }

    pub async fn get_app_info(&self, id: u64) -> Result<Option<NamespaceInfo>, DbErr> {
        NamespaceEntity::find()
            .select_only()
            .column(NamespaceColumn::Id)
            .column(NamespaceColumn::App)
            .column(NamespaceColumn::Cluster)
            .column(NamespaceColumn::Namespace)
            .filter(NamespaceColumn::Id.eq(id))
            .into_model()
            .one(Conn::conn().slaver())
            .await
    }
    pub async fn get_namespace_by_id(&self, id: u64) -> Result<Option<NamespaceModel>, DbErr> {
        NamespaceEntity::find_by_id(id)
            .one(Conn::conn().slaver())
            .await
    }
    pub async fn get_namespace_name(&self, id: u64) -> Result<Option<String>, DbErr> {
        NamespaceEntity::find_by_id(id)
            .select_only()
            .column(NamespaceColumn::Namespace)
            .into_tuple::<String>()
            .one(Conn::conn().slaver())
            .await
    }

    pub async fn is_exist(
        &self,
        app: String,
        cluster: String,
        namespace: String,
    ) -> Result<bool, DbErr> {
        let entity = NamespaceEntity::find()
            .select_only()
            .column(NamespaceColumn::Id)
            .filter(NamespaceColumn::App.eq(app))
            .filter(NamespaceColumn::Cluster.eq(cluster))
            .filter(NamespaceColumn::Namespace.eq(namespace))
            .into_tuple::<u64>()
            .one(Conn::conn().main())
            .await?;
        Ok(entity.is_some())
    }

    pub async fn get_public_namespace_info(
        &self,
        namespace_prefix: Option<String>,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<NamespaceModel>, DbErr> {
        let mut stmt = NamespaceEntity::find().offset(offset).limit(limit);
        if let Some(prefix) = namespace_prefix {
            stmt = stmt.filter(NamespaceColumn::Namespace.starts_with(&prefix))
        }
        stmt.all(Conn::conn().slaver()).await
    }

    pub async fn get_namespace_id(
        &self,
        app_id: String,
        cluster: String,
        namespace: String,
    ) -> Result<Option<u64>, DbErr> {
        NamespaceEntity::find()
            .select_only()
            .column(NamespaceColumn::Id)
            .filter(NamespaceColumn::App.eq(app_id))
            .filter(NamespaceColumn::Cluster.eq(cluster))
            .filter(NamespaceColumn::Namespace.eq(namespace))
            .into_tuple::<u64>()
            .one(Conn::conn().slaver())
            .await
    }
}
