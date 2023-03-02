//! 数据库表结构
pub mod app;
pub mod app_extra;
pub mod cluster;
pub mod common;
pub mod department;
pub mod enums;
pub mod collection;
pub mod item;
pub mod namespace;
pub mod release;
pub mod release_history;
pub mod role;
pub mod role_rule;
pub mod rule;
pub mod user_role;
pub mod users;


pub use cluster::SecretData;
pub use common::{IDu32, ID};
pub use enums::{ItemCategory, Scope};

pub use users::ActiveModel as UsersActive;
pub use users::Column as UsersColumn;
pub use users::Entity as UsersEntity;
pub use users::Model as UsersModel;
pub use users::Claims;
pub use users::UserAuth;

pub use department::ActiveModel as DepartmentActive;
pub use department::Column as DepartmentColumn;
pub use department::Entity as DepartmentEntity;
pub use department::Model as DepartmentModel;

pub use rule::ActiveModel as RuleActive;
pub use rule::Column as RuleColumn;
pub use rule::Entity as RuleEntity;
pub use rule::Model as RuleModel;

pub use role::ActiveModel as RoleActive;
pub use role::Column as RoleColumn;
pub use role::Entity as RoleEntity;
pub use role::Model as RoleModel;

pub use role_rule::ActiveModel as RoleRuleActive;
pub use role_rule::Column as RoleRuleColumn;
pub use role_rule::Entity as RoleRuleEntity;
pub use role_rule::Model as RoleRuleModel;

pub use user_role::ActiveModel as UserRoleActive;
pub use user_role::Column as UserRoleColumn;
pub use user_role::Entity as UserRoleEntity;
pub use user_role::Model as UserRoleModel;

pub use app::ActiveModel as AppActive;
pub use app::Column as AppColumn;
pub use app::Entity as AppEntity;
pub use app::Model as AppModel;

pub use collection::ActiveModel as CollectionActive;
pub use collection::Column as CollectionColumn;
pub use collection::Entity as CollectionEntity;
pub use collection::Model as CollectionModel;

pub use cluster::ActiveModel as ClusterActive;
pub use cluster::Column as ClusterColumn;
pub use cluster::Entity as ClusterEntity;
pub use cluster::Model as ClusterModel;

pub use app_extra::ActiveModel as AppExtraActive;
pub use app_extra::Column as AppExtraColumn;
pub use app_extra::Entity as AppExtraEntity;
pub use app_extra::Model as AppExtraModel;

pub use namespace::ActiveModel as NamespaceActive;
pub use namespace::Column as NamespaceColumn;
pub use namespace::Entity as NamespaceEntity;
pub use namespace::Model as NamespaceModel;

pub use item::ActiveModel as ItemActive;
pub use item::Column as ItemColumn;
pub use item::Entity as ItemEntity;
pub use item::Model as ItemModel;

pub use release::ActiveModel as ReleaseActive;
pub use release::Column as ReleaseColumn;
pub use release::Entity as ReleaseEntity;
pub use release::Model as ReleaseModel;

pub use release_history::ActiveModel as ReleaseHistoryActive;
pub use release_history::Column as ReleaseHistoryColumn;
pub use release_history::Entity as ReleaseHistoryEntity;
pub use release_history::Model as ReleaseHistoryModel;
