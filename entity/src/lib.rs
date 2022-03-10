//! 数据库表结构
pub mod constant;
pub mod dao;
pub mod model;
pub mod utils;
pub mod prelude;

pub use utils::grable_id;
pub use sea_orm as orm;

pub use model::common::{ItemCategory, Premissions, ID};

pub use model::app::ActiveModel as AppActive;
pub use model::app::Column as AppColumn;
pub use model::app::Entity as AppEntity;
pub use model::app::Model as AppModel;

pub use model::cluster::ActiveModel as ClusterActive;
pub use model::cluster::Column as ClusterColumn;
pub use model::cluster::Entity as ClusterEntity;
pub use model::cluster::Model as ClusterModel;

pub use model::app_extend::ActiveModel as AppExtendActive;
pub use model::app_extend::Column as AppExtendColumn;
pub use model::app_extend::Entity as AppExtendEntity;
pub use model::app_extend::Model as AppExtendModel;

pub use model::namespace::ActiveModel as NamespaceActive;
pub use model::namespace::Column as NamespaceColumn;
pub use model::namespace::Entity as NamespaceEntity;
pub use model::namespace::Model as NamespaceModel;

pub use model::item::ActiveModel as ItemActive;
pub use model::item::Column as ItemColumn;
pub use model::item::Entity as ItemEntity;
pub use model::item::Model as ItemModel;
