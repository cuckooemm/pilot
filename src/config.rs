use serde_derive::Deserialize;

#[derive(Clone, Default, Deserialize)]
/// The global configuration
pub struct Config {
    /// The server configuration
    pub server: ServerConfig,

    /// The logger configuration
    pub log: LogConfig,

    pub store: StoreConfig,

    pub harsh: HarshConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let conf_data = std::fs::read_to_string(&path).unwrap();
        let conf: Config = toml::from_str(&conf_data).unwrap();
        conf
    }
}

#[derive(Clone, Default, Deserialize)]
pub struct ServerConfig {
    /// The server IP address
    pub addr: String,
}
#[derive(Clone, Default, Deserialize)]
pub struct HarshConfig {
    pub min_len: usize,
    pub slat: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct DatabaseConfig {
    pub derive: String,
    pub host: String,
    pub user: String,
    pub password: String,
    pub db: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct StoreConfig {
    pub database: DatabaseConfig,
}

#[derive(Clone, Default, Deserialize)]
pub struct LogConfig {
    /// The logging level
    pub level: String,
    /// The log file path
    pub path: String,
}
