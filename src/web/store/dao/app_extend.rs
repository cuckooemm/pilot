use super::Conn;

use entity::namespace::NamespaceItem;
use entity::orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{AppExtendActive, AppExtendColumn, AppExtendEntity, ID};

#[derive(Debug, Clone, Default)]
pub struct AppExtend;

impl AppExtend {
    pub async fn addition(&self, app: AppExtendActive) -> Result<u64, DbErr> {
        let x = AppExtendEntity::insert(app)
            .exec(Conn::conn().main())
            .await?;
        Ok(x.last_insert_id)
    }
    pub async fn is_exist(&self, app_id: String, namespace_name: String) -> Result<bool, DbErr> {
        let entity = AppExtendEntity::find()
            .select_only()
            .column(AppExtendColumn::Id)
            .filter(AppExtendColumn::App.eq(app_id))
            .filter(AppExtendColumn::NamespaceName.eq(namespace_name))
            .into_model::<ID>()
            .one(Conn::conn().main())
            .await?;
        Ok(entity.is_some())
    }
    pub async fn get_app_namespace(&self, app_id: String) -> Result<Vec<NamespaceItem>, DbErr> {
        AppExtendEntity::find()
            .select_only()
            .column_as(AppExtendColumn::NamespaceId, "id")
            .column(AppExtendColumn::NamespaceName)
            .filter(AppExtendColumn::App.eq(app_id))
            .into_model::<NamespaceItem>()
            .all(Conn::conn().slaver())
            .await
    }
}
