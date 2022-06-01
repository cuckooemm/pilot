use super::{master, slaver};

use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};
use entity::users::Status;
use entity::{UsersActive, UsersColumn, UsersEntity, UsersModel, ID};

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

pub async fn get_info(id: u32) -> Result<Option<UsersModel>, DbErr> {
    UsersEntity::find_by_id(id).one(master()).await
}

pub async fn get_use_list(
    dept: Option<u32>,
    status: Status,
    offset: u64,
    limit: u64,
) -> Result<Vec<UsersModel>, DbErr> {
    let mut stmt = UsersEntity::find().offset(offset).limit(limit);
    if let Some(dept) = dept {
        stmt = stmt.filter(UsersColumn::DeptId.eq(dept))
    }
    match status {
        Status::Other => (),
        Status::Normal => stmt = stmt.filter(UsersColumn::DeletedAt.eq(0_u64)),
        Status::Delete => stmt = stmt.filter(UsersColumn::DeletedAt.ne(0_u64)),
    }
    stmt.order_by_desc(UsersColumn::Id).all(slaver()).await
}
