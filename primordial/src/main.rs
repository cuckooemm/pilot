mod config;
mod web;

use config::{Config};
use std::{io, net::SocketAddr};

fn main() {
   
    let conf: Config = Config::from_file(&"primordial/Config.toml");

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
            .await
            .unwrap();
    })
}

