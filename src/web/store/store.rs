use once_cell::sync::OnceCell;

use super::DB;
use crate::config::StoreConfig;

#[derive(Debug, Clone)]
pub struct Store {
    db: DB,
}

static STORE: OnceCell<Store> = OnceCell::new();

pub async fn init_store(store: &StoreConfig) {
    STORE.set(Store::new(store).await).expect("failed to init store");
}

pub fn get_store() -> &'static Store {
    STORE.get().expect("failed to init store")
}

impl Store {
    pub async fn new(store: &StoreConfig) -> Self {
        Self {
            db: DB::new(&store.database).await,
        }
    }
    pub fn db(&self) -> &DB {
        &self.db
    }
}
