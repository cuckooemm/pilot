use super::Conn;

use entity::model::{
    namespace::{NamespaceInfo, NamespaceItem},
    NamespaceActive, NamespaceColumn, NamespaceEntity, NamespaceModel, Scope, ID,
};
use entity::orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};

#[derive(Debug, Clone, Default)]
pub struct Namespace;

impl Namespace {
    pub async fn addition(&self, active: NamespaceActive) -> Result<NamespaceModel, DbErr> {
        active.insert(Conn::conn().main()).await
    }

    pub async fn get_namespace_by_appcluster(
        &self,
        app: String,
        cluster: String,
    ) -> Result<Vec<NamespaceItem>, DbErr> {
        NamespaceEntity::find()
            .select_only()
            .column(NamespaceColumn::Id)
            .column(NamespaceColumn::Namespace)
            .filter(NamespaceColumn::App.eq(app))
            .filter(NamespaceColumn::Cluster.eq(cluster))
            .filter(NamespaceColumn::Scope.eq(Scope::Private))
            .into_model::<NamespaceItem>()
            .all(Conn::conn().slaver())
            .await
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
    pub async fn get_namespace_by_id(&self,id: u64) -> Result<Option<NamespaceModel>,DbErr> {
        NamespaceEntity::find_by_id(id).one(Conn::conn().slaver()).await
    }
    pub async fn get_namespace_name(&self, id: u64) -> Result<Option<String>, DbErr> {
        let ns = NamespaceEntity::find_by_id(id)
            .select_only()
            .column(NamespaceColumn::Namespace)
            .into_model::<NamespaceItem>()
            .one(Conn::conn().slaver())
            .await?;
        Ok(ns.and_then(|n| Some(n.namespace)))
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
            .into_model::<ID>()
            .one(Conn::conn().main())
            .await?;
        Ok(entity.is_some())
    }

    pub async fn get_public_namespace_info(
        &self,
        namespace_prefix: String,
    ) -> Result<Vec<NamespaceInfo>, DbErr> {
        NamespaceEntity::find()
            .select_only()
            .column(NamespaceColumn::Id)
            .column(NamespaceColumn::App)
            .column(NamespaceColumn::Cluster)
            .column(NamespaceColumn::Namespace)
            .filter(NamespaceColumn::Namespace.starts_with(&namespace_prefix))
            .filter(NamespaceColumn::Scope.eq(Scope::Public))
            .into_model::<NamespaceInfo>()
            .all(Conn::conn().slaver())
            .await
    }

    pub async fn get_namespace_id(
        &self,
        app_id: String,
        cluster: String,
        namespace: String,
    ) -> Result<Option<u64>, DbErr> {
        let entity = NamespaceEntity::find()
            .select_only()
            .column(NamespaceColumn::Id)
            .filter(NamespaceColumn::App.eq(app_id))
            .filter(NamespaceColumn::Cluster.eq(cluster))
            .filter(NamespaceColumn::Namespace.eq(namespace))
            .into_model::<ID>()
            .one(Conn::conn().slaver())
            .await?;
        Ok(entity.and_then(|x| Some(x.id)))
    }
}
