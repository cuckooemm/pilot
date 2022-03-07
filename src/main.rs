mod config;
mod web;

use config::Config;
use tokio::signal;
use std::{io, net::SocketAddr};

fn main() {
   
    let conf: Config = Config::from_file(&"Config.toml");

    tracing_subscriber::fmt()
    .with_max_level(conf.log.level.parse::<tracing::Level>().unwrap())
    .with_writer(io::stdout)
    .with_target(true)
    .init();

    let rumtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rumtime.block_on(async {
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