use crate::web::api::backend::department::DepartmentParam;

use super::{master, slaver};

use entity::enums::Status;
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};
use entity::users::{UserAuth, UserItem};
use entity::{
    DepartmentColumn, DepartmentEntity, UsersActive, UsersColumn, UsersEntity, UsersModel, ID,
};

pub async fn add(user: UsersActive) -> Result<u32, DbErr> {
    let r = UsersEntity::insert(user).exec(master()).await?;
    Ok(r.last_insert_id)
}

pub async fn update(user: UsersActive) -> Result<UsersModel, DbErr> {
    user.update(master()).await
}

pub async fn is_exist_account_email(account: String) -> Result<Option<u32>, DbErr> {
    let mut stmt = UsersEntity::find().select_only().column(UsersColumn::Id);
    if account.contains("@") {
        stmt = stmt.filter(UsersColumn::Email.eq(account));
    } else {
        stmt = stmt.filter(UsersColumn::Account.eq(account));
    }
    let id: Option<ID> = stmt.into_model().one(master()).await?;
    match id {
        Some(id) => Ok(Some(id.id as u32)),
        None => Ok(None),
    }
}

pub async fn get_info_by_account(account: String) -> Result<Option<UsersModel>, DbErr> {
    let mut stmt = UsersEntity::find();
    if account.contains("@") {
        stmt = stmt.filter(UsersColumn::Email.eq(account));
    } else {
        stmt = stmt.filter(UsersColumn::Account.eq(account))
    }
    stmt.one(master()).await
}

pub async fn get_user_info(id: u32) -> Result<Option<UserAuth>, DbErr> {
    return UsersEntity::find_by_id(id)
        .select_only()
        .column(UsersColumn::Id)
        .column(UsersColumn::Account)
        .column(UsersColumn::Email)
        .column(UsersColumn::Nickname)
        .column(UsersColumn::DeptId)
        .column(UsersColumn::Level)
        .column(UsersColumn::DeletedAt)
        .into_model::<UserAuth>()
        .one(slaver())
        .await;
}

pub async fn get_info(id: u32) -> Result<Option<UsersModel>, DbErr> {
    UsersEntity::find_by_id(id).one(master()).await
}

pub async fn get_use_list(
    dept: Option<u32>,
    status: Status,
    offset: u64,
    limit: u64,
) -> Result<Vec<UserItem>, DbErr> {
    let mut stmt = UsersEntity::find()
        .select_only()
        .left_join(DepartmentEntity)
        .column(UsersColumn::Id)
        .column(UsersColumn::Account)
        .column(UsersColumn::Email)
        .column(UsersColumn::Nickname)
        .column(UsersColumn::DeptId)
        .column_as(DepartmentColumn::Name, "dept_name")
        .column(UsersColumn::Level)
        .column(UsersColumn::DeletedAt)
        .column(UsersColumn::CreatedAt)
        .column(UsersColumn::UpdatedAt)
        .offset(offset)
        .limit(limit);
    if let Some(dept) = dept {
        stmt = stmt.filter(UsersColumn::DeptId.eq(dept))
    }
    match status {
        Status::Other => (),
        Status::Normal => stmt = stmt.filter(UsersColumn::DeletedAt.eq(0_u64)),
        Status::Delete => stmt = stmt.filter(UsersColumn::DeletedAt.ne(0_u64)),
    }
    stmt.order_by_desc(UsersColumn::Id)
        .into_model::<UserItem>()
        .all(slaver())
        .await
}
