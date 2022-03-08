pub mod db;
pub mod prelude;

use entity::app::Entity as ApplicationEntity;
use entity::item::Entity as ItemEntity;
use entity::cluster::Entity as ClusterEntity;
use entity::app_ns::Entity as AppNsEntity;
use entity::namespace::Entity as ClusterNsEntity;

use entity::sea_orm as orm;