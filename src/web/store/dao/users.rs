use super::Conn;

use entity::enums::Status;
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};
use entity::users::{UserAuth, UserItem};
use entity::{
    DepartmentColumn, DepartmentEntity, UsersActive, UsersColumn, UsersEntity, UsersModel, ID,
};

#[derive(Debug, Clone, Default)]
pub struct Users;

impl Users {
    pub async fn addition(&self, user: UsersActive) -> Result<UsersModel, DbErr> {
        user.insert(Conn::conn().main()).await
    }
    pub async fn update(&self, user: UsersActive) -> Result<UsersModel, DbErr> {
        user.update(Conn::conn().main()).await
    }

    pub async fn is_exist_account_email(&self, account: String) -> Result<bool, DbErr> {
        let mut stmt = UsersEntity::find().select_only().column(UsersColumn::Id);
        if account.contains("@") {
            stmt = stmt.filter(UsersColumn::Email.eq(account));
        } else {
            stmt = stmt.filter(UsersColumn::Account.eq(account));
        }
        stmt.into_model::<ID>()
            .one(Conn::conn().main())
            .await
            .and_then(|id| Ok(id.is_some()))
    }

    pub async fn get_info_by_account(&self, account: String) -> Result<Option<UsersModel>, DbErr> {
        let mut stmt = UsersEntity::find();
        if account.contains("@") {
            stmt = stmt.filter(UsersColumn::Email.eq(account));
        } else {
            stmt = stmt.filter(UsersColumn::Account.eq(account))
        }
        stmt.one(Conn::conn().main()).await
    }

    pub async fn get_user_info(&self, id: u32) -> Result<Option<UserAuth>, DbErr> {
        return UsersEntity::find_by_id(id)
            .select_only()
            .column(UsersColumn::Id)
            .column(UsersColumn::Account)
            .column(UsersColumn::Email)
            .column(UsersColumn::Nickname)
            .column(UsersColumn::DeptId)
            .column(UsersColumn::Level)
            .column(UsersColumn::Status)
            .into_model::<UserAuth>()
            .one(Conn::conn().slaver())
            .await;
    }
    pub async fn get_info(&self, id: u32) -> Result<Option<UsersModel>, DbErr> {
        UsersEntity::find_by_id(id).one(Conn::conn().slaver()).await
    }

    pub async fn get_user_list(
        &self,
        dept: Option<u32>,
        status: Option<Status>,
        (offset, limit): (u64, u64),
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
            .column(UsersColumn::Status)
            .column(UsersColumn::CreatedAt)
            .column(UsersColumn::UpdatedAt)
            .offset(offset)
            .limit(limit);
        if let Some(dept) = dept {
            stmt = stmt.filter(UsersColumn::DeptId.eq(dept));
        }
        if let Some(status) = status {
            stmt = stmt.filter(UsersColumn::Status.eq(status));
        }
        stmt.order_by_desc(UsersColumn::Id)
            .into_model::<UserItem>()
            .all(Conn::conn().slaver())
            .await
    }
}
