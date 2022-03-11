//! 数据库表结构
pub mod app;
pub mod app_extend;
pub mod cluster;
pub mod common;
pub mod constant;
pub mod item;
pub mod namespace;
pub mod release;
pub mod release_record;
pub mod utils;

pub use sea_orm as orm;
pub use utils::grable_id;

pub use cluster::SecretData;
pub use common::{ItemCategory, Premissions, ID};

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
pub use release::Item as ReleaseItem;
pub use release::Model as ReleaseModel;

pub use release_record::ActiveModel as ReleaseRecordActive;
pub use release_record::Column as ReleaseRecordColumn;
pub use release_record::Entity as ReleaseRecordEntity;
pub use release_record::Model as ReleaseRecordModel;
