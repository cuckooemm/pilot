use std::time::Duration;

use entity::orm::{ConnectOptions, Database, DatabaseConnection};
use rand::{thread_rng, RngCore};

use crate::{config::DatabaseCluster, web::store::dao::prelude::init_table};

#[derive(Debug, Clone)]
pub struct DB {
    main: DatabaseConnection,
    slaver: Vec<DatabaseConnection>,
}

fn metrics(info: &entity::orm::metric::Info){
    tracing::info!("database metrics: {:?}",info);
}

impl DB {
    pub async fn new(opt: &DatabaseCluster) -> Self {
        let mut conn = ConnectOptions::new(opt.main.clone().into());
        conn.min_connections(opt.min_connections)
            .connect_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(60))
            .max_lifetime(Duration::from_secs(opt.max_lifetime))
            .sqlx_logging(true);
        
        tracing::info!("connection main databases {}", &conn.get_url());
        let mut main = Database::connect(conn)
            .await
            .expect("failed to connection main database.");
        main.set_metric_callback(metrics);
        let mut slaver: Vec<DatabaseConnection> = Vec::with_capacity(opt.slaver.len());

        // 初始化表
        init_table(&main).await.expect("failed to init table");
        // 初始化从库
        for c in opt.slaver.iter() {
            let mut conn = ConnectOptions::new(c.clone().into());
            conn.min_connections(opt.min_connections)
                .connect_timeout(Duration::from_secs(3))
                .idle_timeout(Duration::from_secs(60))
                .max_lifetime(Duration::from_secs(opt.max_lifetime))
                .sqlx_logging(true);
            tracing::info!("connection slaver databases {}", &conn.get_url());

            slaver.push(
                Database::connect(conn)
                    .await
                    .expect(&format!("failed to connection slaver database.")),
            );
        }
        Self { main, slaver }
    }
    // 获取只读链接
    pub fn slaver(&self) -> &DatabaseConnection {
        if self.slaver.is_empty() {
            return &self.main;
        }
        self.slaver
            .get(thread_rng().next_u32() as usize % self.slaver.len())
            .unwrap_or(&self.main)
    }

    pub fn master(&self) -> &DatabaseConnection {
        return &self.main;
    }
}
