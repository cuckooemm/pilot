mod config;
mod web;

use config::Config;
use std::{io, net::SocketAddr, time::Duration};
use tokio::signal;

fn main() {
    let conf: Config = Config::from_file(&"Config.toml");
    tracing_subscriber::fmt()
        .with_max_level(conf.log.level.parse::<tracing::Level>().unwrap())
        .with_writer(io::stdout)
        .with_target(true)
        .init();

    entity::utils::init_harsh(conf.harsh.min_len, &conf.harsh.slat);
    let rumtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rumtime.block_on(async {
        init_store(&conf.store).await;
        let router = web::route::init_router(conf.store).await;
        let addr: SocketAddr = conf.server.addr.parse().unwrap();
        tracing::info!("listening on {}", &addr);

        axum::Server::bind(&addr)
            .http1_keepalive(true)
            .serve(router.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    })
}

// 初始化全局变量
async fn init_store(conf: &config::StoreConfig) {
    let addr = format!(
        "{}://{}:{}@{}/{}?useUnicode=ture&characterEncoding=UTF-8",
        conf.database.derive,
        conf.database.user,
        conf.database.password,
        conf.database.host,
        conf.database.db
    );
    let mut opt = entity::orm::ConnectOptions::new(addr);
    opt.min_connections(1)
        .connect_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(60))
        .max_lifetime(Duration::from_secs(300))
        .sqlx_logging(true);
    tracing::info!("connection databases {}", &opt.get_url());
    if let Err(e) = entity::prelude::init_orm(opt).await {
        panic!("failed to connection database. err: {}", e)
    }
}

// 监听退出信号
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}
