pub mod app;
pub mod app_extend;
pub mod cluster;
pub mod department;
pub mod item;
pub mod namespace;
pub mod publication;
pub mod publication_record;
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
