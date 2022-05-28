pub mod app;
pub mod app_extend;
pub mod cluster;
pub mod department;
pub mod item;
pub mod namespace;
pub mod release;
pub mod release_history;
pub mod role;
pub mod rule;
pub mod user_role;
pub mod users;

use entity::orm::DatabaseConnection;

use super::store::get_store;

fn master() -> &'static DatabaseConnection {
    get_store().db().master()
}

fn slaver() -> &'static DatabaseConnection {
    get_store().db().slaver()
}
