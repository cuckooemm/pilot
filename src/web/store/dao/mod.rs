pub mod app;
pub mod app_extend;
pub mod cluster;
pub mod item;
pub mod namespace;
pub mod prelude;
pub mod release;
pub mod release_record;

use entity::orm::DatabaseConnection;

use super::store::get_store;

fn master() -> &'static DatabaseConnection {
    get_store().db().master()
}

fn slaver() -> &'static DatabaseConnection {
    get_store().db().slaver()
}
