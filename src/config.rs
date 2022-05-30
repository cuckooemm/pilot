use std::env;

use once_cell::sync::OnceCell;

static CONF: OnceCell<Config> = OnceCell::new();

#[derive(Debug, Clone)]
/// The global configuration
pub struct Config {
    /// The server configuration
    pub server: ServerConfig,

    /// The logger configuration
    pub log: LogConfig,

    pub store: StoreConfig,

    pub harsh: HarshConfig,

    pub jwt_secret: String,
}

pub fn get_store() -> &'static StoreConfig {
    &CONF.get().unwrap().store
}

pub fn get_log() -> &'static LogConfig {
    &CONF.get().unwrap().log
}

pub fn get_harsh() -> &'static HarshConfig {
    &CONF.get().unwrap().harsh
}

pub fn get_server() -> &'static ServerConfig {
    &CONF.get().unwrap().server
}

pub fn get_jwt_secret() -> &'static String {
    &CONF.get().unwrap().jwt_secret
}

impl Config {
    pub fn init_env() {
        let addr = env::var("PILOT_LISTEN_ADDR").unwrap_or("0.0.0.0:8000".to_owned());
        let log_level = env::var("PILOT_LOG_LEVEL")
            .and_then(|s| Ok(s.parse::<tracing::Level>().unwrap_or(tracing::Level::WARN)))
            .unwrap_or(tracing::Level::WARN);

        let main_db_host = env::var("PILOT_DB_MASTER_HOST").expect("Specify the master DB host");
        let slaver_db_host = env::var("PILOT_DB_SLAVER_HOST")
            .map(|s| {
                s.split(",")
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or(vec![]);

        let min_connections = env::var("PILOT_MIN_CONNECTIONS")
            .map(|s| s.parse::<u32>().unwrap_or(3))
            .unwrap_or(3);
        let max_lifetime = env::var("PILOT_MAX_LIFETIME")
            .map(|s| s.parse::<u64>().unwrap_or(300))
            .unwrap_or(300);

        let hasher_slat =
            env::var("PILOT_HASHER_SLAT").unwrap_or("qpwoeirutyalskdjfhgmznxbcv".to_owned());
        let jwt_secret =
            env::var("PILOT_JWT_SECRET").unwrap_or("qpwoeirutyalskdjfhgmznxbcv".to_owned());

        let conf = Self {
            server: ServerConfig { addr },
            log: LogConfig { level: log_level },
            store: StoreConfig {
                database: DatabaseCluster {
                    main: main_db_host,
                    slaver: slaver_db_host,
                    min_connections,
                    max_lifetime,
                },
            },
            harsh: HarshConfig {
                min_len: 16,
                slat: hasher_slat,
            },
            jwt_secret,
        };
        tracing::info!("load config: {:?}", &conf);
        CONF.set(conf).ok().unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// The server IP address
    pub addr: String,
}
#[derive(Debug, Clone)]
pub struct HarshConfig {
    pub min_len: usize,
    pub slat: String,
}

#[derive(Debug, Clone)]
pub struct StoreConfig {
    pub database: DatabaseCluster,
}

#[derive(Debug, Clone, Default)]
pub struct DatabaseCluster {
    pub main: String,
    pub slaver: Vec<String>,
    pub min_connections: u32,
    pub max_lifetime: u64,
}

#[derive(Debug, Clone)]
pub struct LogConfig {
    /// The logging level
    pub level: tracing::Level,
}
