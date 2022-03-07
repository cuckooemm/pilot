pub mod app;
pub mod app_ns;
pub mod check;
pub mod cluster;
pub mod common;
pub mod config;
pub mod item;
pub mod namespace;
pub mod response;

use entity::sea_orm as orm;

type APIResult<T> = std::result::Result<T, response::APIError>;

use super::store::db::StoreStats;

use entity::common::{ItemCategory, Premissions, ID};

use entity::app::ActiveModel as AppActive;
use entity::app::Column as AppColumn;
use entity::app::Entity as AppEntity;
use entity::app::Model as AppModel;

use entity::cluster::ActiveModel as ClusterActive;
use entity::cluster::Column as ClusterColumn;
use entity::cluster::Entity as ClusterEntity;
use entity::cluster::Model as ClusterModel;

use entity::app_ns::ActiveModel as AppNsActive;
use entity::app_ns::Column as AppNsColumn;
use entity::app_ns::Entity as AppNsEntity;
use entity::app_ns::Model as AppNsModel;

use entity::namespace::ActiveModel as NamespaceActive;
use entity::namespace::Column as NamespaceColumn;
use entity::namespace::Entity as NamespaceEntity;
use entity::namespace::Model as NamespaceModel;

use entity::item::ActiveModel as ItemActive;
use entity::item::Column as ItemColumn;
use entity::item::Entity as ItemEntity;
use entity::item::Model as ItemModel;
