use anyhow::Result;
use clap::Parser;

use tokio::net::TcpListener;

use modules::{
    logger,
    settings::{args, config},
};

mod network;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init_logger();

    let args = args::Args::parse();
    let config = config::init_config(config::ClientOrServer::new(args.ip, args.port), args.config)?;

    let listener = TcpListener::bind(format!("{}:{}", config.ip, config.port)).await?;
    tracing::info!("Server started");

    let start = start(listener);
    let exit = tokio::signal::ctrl_c();

    tokio::select! {
        _ = start => (),
        _ = exit => (),
    }

    tracing::info!("Server exited");
    Ok(())
}

async fn start(listener: TcpListener) -> Result<()> {
    loop {
        let (stream, address) = listener.accept().await?;
        tokio::spawn(async move {
            network::handle_connection(stream, address).await;
        });
    }
}
