use super::Conn;

use entity::ID;
use entity::model::{
    AppExtraActive, AppExtraColumn, AppExtraEntity, NamespaceColumn, NamespaceEntity,
    NamespaceModel,
};
use entity::orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};

#[derive(Debug, Clone, Default)]
pub struct AppExtra;

impl AppExtra {
    pub async fn addition(&self, app: AppExtraActive) -> Result<u64, DbErr> {
        let x = AppExtraEntity::insert(app)
            .exec(Conn::conn().main())
            .await?;
        Ok(x.last_insert_id)
    }
    pub async fn deleted(&self, app: String, id: u64) -> Result<(), DbErr> {
        let _ = AppExtraEntity::delete_many()
            .filter(AppExtraColumn::App.eq(app))
            .filter(AppExtraColumn::NamespaceId.eq(id))
            .exec(Conn::conn().main())
            .await?;
        Ok(())
    }
    pub async fn is_exist(&self, app: String, namespace_id: u64) -> Result<bool, DbErr> {
        let entity = AppExtraEntity::find()
            .select_only()
            .column(AppExtraColumn::Id)
            .filter(AppExtraColumn::App.eq(app))
            .filter(AppExtraColumn::NamespaceId.eq(namespace_id))
            .into_model::<ID>()
            .one(Conn::conn().main())
            .await?;
        Ok(entity.is_some())
    }
    pub async fn get_app_namespace(
        &self,
        app: String,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<NamespaceModel>, DbErr> {
        let id = AppExtraEntity::find()
            .select_only()
            .column_as(AppExtraColumn::NamespaceId, "id")
            .filter(AppExtraColumn::App.eq(app))
            .offset(offset)
            .limit(limit)
            .into_model::<ID>()
            .all(Conn::conn().slaver())
            .await?;

        let ids: Vec<u64> = id.into_iter().map(|id| id.id).collect();
        NamespaceEntity::find()
            .filter(NamespaceColumn::Id.is_in(ids))
            .all(Conn::conn().slaver())
            .await
    }
}
