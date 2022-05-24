//! 数据库表结构
pub mod app;
pub mod app_extend;
pub mod cluster;
pub mod common;
pub mod constant;
pub mod department;
pub mod item;
pub mod namespace;
pub mod publication;
pub mod publication_record;
pub mod release;
pub mod role;
pub mod role_rule;
pub mod rule;
pub mod user_role;
pub mod users;
pub mod utils;

pub use sea_orm as orm;

pub use utils::grable_id;
pub use utils::grable_id_u32;

pub use cluster::SecretData;
pub use common::{IDu32, ItemCategory, Scope, ID};

pub use users::ActiveModel as UsersActive;
pub use users::Column as UsersColumn;
pub use users::Entity as UsersEntity;
pub use users::Model as UsersModel;

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

pub use cluster::ActiveModel as ClusterActive;
pub use cluster::Column as ClusterColumn;
pub use cluster::Entity as ClusterEntity;
pub use cluster::Model as ClusterModel;

pub use app_extend::ActiveModel as AppExtendActive;
pub use app_extend::Column as AppExtendColumn;
pub use app_extend::Entity as AppExtendEntity;
pub use app_extend::Model as AppExtendModel;

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

pub use publication::ActiveModel as PublicationActive;
pub use publication::Column as PublicationColumn;
pub use publication::Entity as PublicationEntity;
pub use publication::Item as PublicationItem;
pub use publication::Model as PublicationModel;

pub use publication_record::ActiveModel as PublicationRecordActive;
pub use publication_record::Column as PublicationRecordColumn;
pub use publication_record::Entity as PublicationRecordEntity;
pub use publication_record::Model as PublicationRecordModel;
