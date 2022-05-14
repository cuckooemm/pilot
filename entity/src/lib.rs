//! 数据库表结构
pub mod app;
pub mod app_extend;
pub mod cluster;
pub mod common;
pub mod constant;
pub mod item;
pub mod namespace;
pub mod publication;
pub mod publication_record;
pub mod utils;
pub mod user;

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

pub use publication::ActiveModel as PublicationActive;
pub use publication::Column as PublicationColumn;
pub use publication::Entity as PublicationEntity;
pub use publication::Item as PublicationItem;
pub use publication::Model as PublicationModel;

pub use publication_record::ActiveModel as PublicationRecordActive;
pub use publication_record::Column as PublicationRecordColumn;
pub use publication_record::Entity as PublicationRecordEntity;
pub use publication_record::Model as PublicationRecordModel;
