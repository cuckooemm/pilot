use super::master;

use entity::orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::{UsersActive, UsersColumn, UsersEntity, UsersModel, ID};

pub async fn add(user: UsersActive) -> Result<u32, DbErr> {
    let r = UsersEntity::insert(user).exec(master()).await?;
    Ok(r.last_insert_id)
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
