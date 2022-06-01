use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "super::grable_id_u32")]
    pub id: u32, // 用户ID
    pub account: String,  // 登录用户名
    pub email: String,    // 邮箱
    pub nickname: String, // 用户名
    #[serde(skip)]
    pub password: String, // 密码
    #[serde(serialize_with = "super::grable_id_u32")]
    pub dept_id: u32, // 部门
    pub dept_name: String, // 部门名称
    pub level: UserLevel, // 帐号等级
    #[serde(serialize_with = "super::format_time")]
    //     skip_serializing_if = "super::is_zero"
    pub deleted_at: u64, // 删除时间
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

#[derive(
    Debug, Clone, PartialEq, PartialOrd, EnumIter, DeriveActiveEnum, Deserialize, Serialize, Copy,
)]
#[sea_orm(rs_type = "u16", db_type = "SmallUnsigned")]
pub enum UserLevel {
    #[sea_orm(num_value = 0)]
    #[serde(rename = "normal")]
    Normal,
    #[sea_orm(num_value = 10)]
    #[serde(rename = "dept_admin")]
    DeptAdmin,
    #[sea_orm(num_value = 100)]
    #[serde(rename = "admin")]
    Admin,
}

impl From<String> for UserLevel {
    fn from(str: String) -> Self {
        match str.to_lowercase().as_str() {
            "admin" => Self::Admin,
            "dept_admin" => Self::DeptAdmin,
            _ => Self::Normal,
        }
    }
}

pub enum Status {
    Normal,
    Delete,
    Other,
}

impl From<String> for Status {
    fn from(str: String) -> Self {
        match str.trim().to_lowercase().as_str() {
            "delete" => Self::Delete,
            "normal" => Self::Normal,
            _ => Self::Other,
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Normal
    }
}
