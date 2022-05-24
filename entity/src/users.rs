use crate::grable_id_u32;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "grable_id_u32")]
    pub id: u32, // 用户ID
    pub account: String,                  // 登录用户名
    pub email: String,                    // 邮箱
    pub nickname: String,                 // 用户名
    pub password: String,                 // 密码
    pub org_id: u32,                      // 用户部门
    pub level: UserLevel,                 // 帐号等级
    pub deleted_at: u64,                  // 删除时间
    pub created_at: DateTimeWithTimeZone, // 创建时间
    pub updated_at: DateTimeWithTimeZone, // 更新时间
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "u16", db_type = "SmallUnsigned")]
pub enum UserLevel {
    #[sea_orm(num_value = 0)]
    Normal,
    #[sea_orm(num_value = 10)]
    OrgAdmin,
    #[sea_orm(num_value = 100)]
    Admin,
}
