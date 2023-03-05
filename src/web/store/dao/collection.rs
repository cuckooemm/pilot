use super::Conn;

use axum::extract::State;
use entity::common::enums::Status;
use entity::model::{
    app::AppItem, AppColumn, AppEntity, CollectionActive, CollectionColumn, CollectionEntity,
};
use entity::model::{AppModel, CollectionModel};
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use entity::ID;

#[derive(Debug, Clone, Default)]
pub struct Collection;

impl Collection {
    pub async fn addition(&self, app_id: u32, user_id: u32) -> Result<u64, DbErr> {
        let x = CollectionEntity::insert(CollectionActive {
            user_id: Set(user_id),
            app_id: Set(app_id),
            ..Default::default()
        })
        .exec(Conn::conn().main())
        .await?;
        Ok(x.last_insert_id)
    }

    pub async fn save(&self, active: CollectionActive) -> Result<bool, DbErr> {
        active.save(Conn::conn().main()).await?;
        Ok(true)
    }
    pub async fn get_collection(
        &self,
        app_id: u32,
        user_id: u32,
    ) -> Result<Option<CollectionModel>, DbErr> {
        CollectionEntity::find()
            .filter(CollectionColumn::UserId.eq(user_id))
            .filter(CollectionColumn::AppId.eq(app_id))
            .one(Conn::conn().main())
            .await
    }
    pub async fn is_exist(&self, app_id: u32, user_id: u32) -> Result<bool, DbErr> {
        let model: Option<u64> = CollectionEntity::find()
            .select_only()
            .column(CollectionColumn::Id)
            .filter(CollectionColumn::UserId.eq(user_id))
            .filter(CollectionColumn::AppId.eq(app_id))
            .into_tuple()
            .one(Conn::conn().main())
            .await?;
        Ok(model.is_some())
    }

    pub async fn get_apps(
        &self,
        user_id: u32,
        status: Option<Status>,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<AppModel>, DbErr> {
        let mut stmt = CollectionEntity::find()
            .select_only()
            .columns([
                AppColumn::App,
                AppColumn::Name,
                AppColumn::Describe,
                AppColumn::DepartmentId,
                AppColumn::Status,
                AppColumn::CreatedAt,
                AppColumn::UpdatedAt,
            ])
            .left_join(AppEntity)
            .filter(CollectionColumn::UserId.eq(user_id));
        if let Some(s) = status {
            stmt = stmt.filter(AppColumn::Status.eq(s))
        }
        stmt.order_by_desc(CollectionColumn::Id)
            .offset(offset)
            .limit(limit)
            .into_model::<AppModel>()
            .all(Conn::conn().slaver())
            .await
    }
}
