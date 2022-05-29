use super::{master, slaver};

use entity::{
    app::AppItem,
    orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set},
    AppColumn, AppEntity, FavoriteActive, FavoriteColumn, FavoriteEntity, ID,
};

pub async fn add(app_id: u32, user_id: u32) -> Result<u64, DbErr> {
    let x = FavoriteEntity::insert(FavoriteActive {
        user_id: Set(user_id),
        app_id: Set(app_id),
        ..Default::default()
    })
    .exec(master())
    .await?;
    Ok(x.last_insert_id)
}

pub async fn is_exist(app_id: u32, user_id: u32) -> Result<bool, DbErr> {
    let x = FavoriteEntity::find()
        .select_only()
        .column(FavoriteColumn::Id)
        .filter(FavoriteColumn::UserId.eq(user_id))
        .filter(FavoriteColumn::AppId.eq(app_id))
        .filter(FavoriteColumn::DeletedAt.eq(0_u64))
        .into_model::<ID>()
        .one(master())
        .await?;
    Ok(x.is_some())
}

pub async fn get_app(user_id: u32, offset: u64, limit: u64) -> Result<Vec<AppItem>, DbErr> {
    FavoriteEntity::find()
        .select_only()
        .column(AppColumn::AppId)
        .column(AppColumn::Name)
        .left_join(AppEntity)
        .filter(FavoriteColumn::UserId.eq(user_id))
        .filter(FavoriteColumn::DeletedAt.eq(0_u64))
        .filter(AppColumn::DeletedAt.eq(0_u64))
        .order_by_desc(FavoriteColumn::Id) // 最后收藏的放前面
        .offset(offset)
        .limit(limit)
        .into_model::<AppItem>()
        .all(slaver())
        .await
}
