mod config;
mod web;

use config::Config;
use std::{io, net::SocketAddr, time::Duration};
use tokio::signal;

use crate::web::store::store::init_store;

fn main() {
    dotenv::dotenv().ok();
    Config::init_env();

    tracing_subscriber::fmt()
        .with_max_level(config::get_log().level)
        .with_writer(io::stdout)
        .with_target(true)
        .init();
    let harsh = config::get_harsh();
    entity::utils::init_harsh(harsh.min_len, &harsh.slat);

    let rumtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rumtime.block_on(async {
        init_store(&config::get_store()).await;

        let router = web::route::init_router().await;
        let svc = config::get_server();
        let addr: SocketAddr = svc.addr.parse().unwrap();
        tracing::info!("listening on {}", &addr);

        axum::Server::bind(&addr)
            .http1_keepalive(true)
            .tcp_keepalive(Some(Duration::from_secs(90)))
            .serve(router.into_make_service())
            // .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();

        tracing::info!("exit server...");
    })
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
