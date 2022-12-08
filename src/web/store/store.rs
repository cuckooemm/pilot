use super::{
    cache::CacheItem,
    dao::{Conn, Dao},
};

use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct Store {
    dao: Dao,
    memch: CacheItem,
}

impl Store {
    pub async fn new() -> Self {
        Conn::connection().await;
        Self {
            dao: Dao::new(),
            memch: CacheItem::new(),
        }
    }
}
