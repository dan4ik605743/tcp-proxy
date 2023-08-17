use std::sync::Arc;

use anyhow::Result;
use clap::Parser;

use tokio::net::TcpListener;

use modules::{logger, settings::config};

mod args;
mod proxy;

use proxy::Proxy;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init_logger();

    let args = args::Args::parse();
    let config = config::init_config(
        config::Proxy::new(
            args.ip,
            args.port,
            args.server_ip,
            args.server_port,
            args.connection_restrictions,
            args.timeout_message,
        ),
        args.config,
    )?;

    let listener = TcpListener::bind(format!("{}:{}", config.ip, config.port)).await?;
    tracing::info!("Proxy started");
    let proxy = Arc::new(Proxy::new(
        config.connection_restrictions,
        config.server_ip,
        config.server_port,
    ));

    let start = start(listener, proxy, config.timeout_message);
    let exit = tokio::signal::ctrl_c();

    tokio::select! {
        _ = start => (),
        _ = exit => (),
    }

    tracing::info!("Proxy exited");
    Ok(())
}

async fn start(listener: TcpListener, proxy: Arc<Proxy>, timeout_message: u64) -> Result<()> {
    loop {
        let (stream, address) = listener.accept().await?;
        let proxy = Arc::clone(&proxy);

        tokio::spawn(async move {
            proxy
                .handle_connection(stream, address, timeout_message)
                .await;
        });
    }
}
