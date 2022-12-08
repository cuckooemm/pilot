use super::Conn;

use entity::{
    app::AppItem,
    orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set},
    AppColumn, AppEntity, CollectionActive, CollectionColumn, CollectionEntity, ID,
};

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

    pub async fn is_exist(&self, app_id: u32, user_id: u32) -> Result<bool, DbErr> {
        let x = CollectionEntity::find()
            .select_only()
            .column(CollectionColumn::Id)
            .filter(CollectionColumn::UserId.eq(user_id))
            .filter(CollectionColumn::AppId.eq(app_id))
            .into_model::<ID>()
            .one(Conn::conn().main())
            .await?;
        Ok(x.is_some())
    }

    pub async fn get_app(
        &self,
        user_id: u32,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<AppItem>, DbErr> {
        CollectionEntity::find()
            .select_only()
            .column(AppColumn::App)
            .column(AppColumn::Name)
            .left_join(AppEntity)
            .filter(CollectionColumn::UserId.eq(user_id))
            .order_by_desc(CollectionColumn::Id) // 最后收藏的放前面
            .offset(offset)
            .limit(limit)
            .into_model::<AppItem>()
            .all(Conn::conn().slaver())
            .await
    }
}
