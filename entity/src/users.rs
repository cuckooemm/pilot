use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "super::confuse")]
    pub id: u32, // 用户ID
    pub account: String,  // 登录用户名
    pub email: String,    // 邮箱
    pub nickname: String, // 用户名
    #[serde(skip)]
    pub password: String, // 密码
    #[serde(serialize_with = "super::confuse")]
    pub dept_id: u32, // 部门
    pub level: UserLevel, // 帐号等级
    #[serde(
        serialize_with = "super::format_time",
        skip_serializing_if = "super::is_zero"
    )]
    pub deleted_at: u64, // 删除时间
    pub created_at: DateTimeWithTimeZone, // 创建时间
    pub updated_at: DateTimeWithTimeZone, // 更新时间
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Department,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Department => Entity::belongs_to(super::DepartmentEntity)
                .from(Column::DeptId)
                .to(super::DepartmentColumn::Id)
                .into(),
        }
    }
}
impl Related<super::DepartmentEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Department.def()
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
    Ban,
    #[sea_orm(num_value = 1)]
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

#[derive(FromQueryResult, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserItem {
    #[serde(serialize_with = "super::confuse")]
    pub id: u32, // 用户ID
    pub account: String,  // 登录用户名
    pub email: String,    // 邮箱
    pub nickname: String, // 用户名
    #[serde(serialize_with = "super::confuse")]
    pub dept_id: u32, // 部门
    pub dept_name: String,
    pub level: UserLevel, // 帐号等级
    #[serde(
        serialize_with = "super::format_time",
        skip_serializing_if = "super::is_zero"
    )]
    pub deleted_at: u64, // 删除时间
    pub created_at: DateTimeWithTimeZone, // 创建时间
    pub updated_at: DateTimeWithTimeZone, // 更新时间
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Claims {
    pub uid: u32,
    pub renewal: i64,
    pub exp: i64,
}

#[derive(FromQueryResult, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserAuth {
    pub id: u32,          // 用户ID
    pub account: String,  // 登录用户名
    pub email: String,    // 邮箱
    pub nickname: String, // 用户名
    pub dept_id: u32,     // 部门
    pub level: UserLevel, // 帐号
    pub deleted_at: u64, // 删除时间
}
