use super::Conn;

use entity::enums::Status;
use entity::orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use entity::user_role::UserRoleID;
use entity::{UserRoleActive, UserRoleColumn, UserRoleEntity};

#[derive(Debug, Clone, Default)]
pub struct UserRule;

impl UserRule {
    pub async fn addition(&self, user: UserRoleActive) -> Result<u64, DbErr> {
        let r = UserRoleEntity::insert(user)
            .exec(Conn::conn().main())
            .await?;
        Ok(r.last_insert_id)
    }

    pub async fn get_user_role(&self, user_id: u32) -> Result<Vec<u32>, DbErr> {
        let role_ids = UserRoleEntity::find()
            .select_only()
            .column(UserRoleColumn::RoleId)
            .filter(UserRoleColumn::UserId.eq(user_id))
            .filter(UserRoleColumn::Status.eq(Status::Normal))
            .into_model::<UserRoleID>()
            .all(Conn::conn().slaver())
            .await?;
        let role_ids = role_ids
            .into_iter()
            .map(|id| id.role_id)
            .collect::<Vec<u32>>();
        Ok(role_ids)
    }
}
