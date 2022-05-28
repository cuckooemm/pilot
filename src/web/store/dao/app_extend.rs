use super::{master, slaver};

use entity::namespace::NamespaceItem;
use entity::orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{AppExtendActive, AppExtendColumn, AppExtendEntity, ID};

pub async fn add(app: AppExtendActive) -> Result<u64, DbErr> {
    let x = AppExtendEntity::insert(app).exec(master()).await?;
    Ok(x.last_insert_id)
}

pub async fn is_exist(app_id: String, namespace_name: String) -> Result<bool, DbErr> {
    let entity = AppExtendEntity::find()
        .select_only()
        .column(AppExtendColumn::Id)
        .filter(AppExtendColumn::AppId.eq(app_id))
        .filter(AppExtendColumn::NamespaceName.eq(namespace_name))
        .filter(AppExtendColumn::DeletedAt.eq(0_u64))
        .into_model::<ID>()
        .one(master())
        .await?;
    Ok(entity.is_some())
}

pub async fn get_app_namespace(app_id: String) -> Result<Vec<NamespaceItem>, DbErr> {
    AppExtendEntity::find()
        .select_only()
        .column_as(AppExtendColumn::NamespaceId, "id")
        .column(AppExtendColumn::NamespaceName)
        .filter(AppExtendColumn::AppId.eq(app_id))
        .filter(AppExtendColumn::DeletedAt.eq(0_u64))
        .into_model::<NamespaceItem>()
        .all(slaver())
        .await
}
