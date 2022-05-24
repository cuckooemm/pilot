use std::time::Duration;

use entity::orm::{ConnectOptions, Database, DatabaseConnection};
use rand::{thread_rng, RngCore};

use crate::{config::DatabaseCluster};

#[derive(Debug, Clone)]
pub struct DB {
    main: DatabaseConnection,
    slavers: Vec<DatabaseConnection>,
}

fn master_metrics(info: &entity::orm::metric::Info) {
    let labels = [("success", (!info.failed).to_string()), ("model","slaver".to_owned())];
    metrics::histogram!(
        "database_duration_seconds",
        info.elapsed.as_secs_f64(),
        &labels
    );
    metrics::increment_counter!("database_requests_total", &labels);
    // tracing::info!("database metrics: {:#?}", info);
}

fn slaver_metrics(info: &entity::orm::metric::Info) {
    let labels = [("success", (!info.failed).to_string()), ("model","slaver".to_owned())];
    metrics::histogram!(
        "database_duration_seconds",
        info.elapsed.as_secs_f64(),
        &labels
    );
    metrics::increment_counter!("database_requests_total", &labels);
    // tracing::info!("database metrics: {:#?}", info);
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
        main.set_metric_callback(master_metrics);
        let mut slavers: Vec<DatabaseConnection> = Vec::with_capacity(opt.slaver.len());

        // 初始化从库
        for c in opt.slaver.iter() {
            let mut conn = ConnectOptions::new(c.clone().into());
            conn.min_connections(opt.min_connections)
                .connect_timeout(Duration::from_secs(3))
                .idle_timeout(Duration::from_secs(60))
                .max_lifetime(Duration::from_secs(opt.max_lifetime))
                .sqlx_logging(true);
            tracing::info!("connection slaver databases {}", &conn.get_url());
            let mut slaver = Database::connect(conn)
                .await
                .expect(&format!("failed to connection slaver database."));
            slaver.set_metric_callback(slaver_metrics);
            slavers.push(slaver);
        }
        Self { main, slavers }
    }
    // 获取只读链接
    pub fn slaver(&self) -> &DatabaseConnection {
        if self.slavers.is_empty() {
            return &self.main;
        }
        self.slavers
            .get(thread_rng().next_u32() as usize % self.slavers.len())
            .unwrap_or(&self.main)
    }

    pub fn master(&self) -> &DatabaseConnection {
        return &self.main;
    }
}
