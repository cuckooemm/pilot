use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::StoreConfig;

#[derive(Clone)]
pub struct StoreStats {
    pub db: DatabaseConnection,
}

impl StoreStats {
    pub async fn new(conf: StoreConfig) -> Self {
        let addr = format!(
            "{}://{}:{}@{}/{}",
            conf.database.derive,
            conf.database.user,
            conf.database.password,
            conf.database.host,
            conf.database.db
        );
        let mut opt = ConnectOptions::new(addr);
        opt.min_connections(3)
            .connect_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(60))
            .sqlx_logging(true);
        let db = Database::connect(opt)
            .await
            .expect("failed to connection databases");
        // 初始化表
        super::prelude::create_table(&db).await;

        StoreStats { db }
    }
}
