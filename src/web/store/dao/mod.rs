pub mod app;
pub mod app_extra;
pub mod cluster;
pub mod collection;
pub mod department;
pub mod item;
pub mod namespace;
pub mod release;
pub mod rule;
pub mod user_role;
pub mod users;

use crate::config::get_store;

use entity::orm::{Database, DatabaseConnection, DbConn};
use rand::{thread_rng, RngCore};
use tokio::sync::OnceCell;

static DB_CONN: OnceCell<Conn> = OnceCell::const_new();

#[derive(Debug, Clone)]
pub struct Conn {
    main: DbConn,
    slavers: Vec<DbConn>,
}

impl Conn {
    pub fn main(&self) -> &DbConn {
        &self.main
    }
    pub fn slaver(&self) -> &DbConn {
        if self.slavers.is_empty() {
            return &self.main;
        }
        self.slavers
            .get(thread_rng().next_u32() as usize % self.slavers.len())
            .unwrap_or(&self.main)
    }
    pub async fn connection() {
        let conf = &get_store().database;
        let conn = conf.get_connect_options(conf.main.clone());
        tracing::info!("connection main databases {}", &conn.get_url());
        let mut main = Database::connect(conn)
            .await
            .expect("failed to connection main database.");
        main.set_metric_callback(main_metrics);

        let mut slavers: Vec<DatabaseConnection> = Vec::with_capacity(conf.slaver.len());
        // 初始化从库
        for c in conf.slaver.iter() {
            let conn = conf.get_connect_options(c.clone());
            tracing::info!("connection slaver databases {}", &conn.get_url());
            let mut slaver = Database::connect(conn)
                .await
                .expect("failed to connection slaver database.");
            slaver.set_metric_callback(slaver_metrics);
            slavers.push(slaver);
        }
        DB_CONN.set(Conn { main, slavers }).unwrap();
    }
    pub fn conn() -> &'static Conn {
        DB_CONN.get().unwrap()
    }
}

#[derive(Clone, Default, Debug)]
pub struct Dao {
    pub users: users::Users,
    pub app_extra: app_extra::AppExtra,
    pub app: app::App,
    pub cluster: cluster::Cluster,
    pub collection: collection::Collection,
    pub department: department::Department,
    pub item: item::Item,
    pub namespace: namespace::Namespace,
    pub release: release::Release,
    pub rule: rule::Rule,
    pub user_role: user_role::UserRule,
}

impl Dao {
    pub fn new() -> Self {
        Self::default()
    }
}

fn main_metrics(info: &entity::orm::metric::Info) {
    let labels = [
        ("success", (!info.failed).to_string()),
        ("model", "slaver".to_owned()),
    ];
    metrics::histogram!(
        "database_duration_seconds",
        info.elapsed.as_secs_f64(),
        &labels
    );
    metrics::increment_counter!("database_requests_total", &labels);
}

fn slaver_metrics(info: &entity::orm::metric::Info) {
    let labels = [
        ("success", (!info.failed).to_string()),
        ("model", "slaver".to_owned()),
    ];
    metrics::histogram!(
        "database_duration_seconds",
        info.elapsed.as_secs_f64(),
        &labels
    );
    metrics::increment_counter!("database_requests_total", &labels);
}
