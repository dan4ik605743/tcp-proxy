use anyhow::Result;
use clap::Parser;

use modules::{
    async_utils, logger,
    network::{connection, message::Msg},
    settings::{args, config},
};

mod network;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init_logger();

    let args = args::ClientOrServer::parse();
    let config = config::init_config(config::ClientOrServer::new(args.ip, args.port), args.config)?;

    let lost_msg: Option<Msg> = None;

    start(&config.ip, config.port, lost_msg).await
}

async fn start(ip: &str, port: u32, mut lost_msg: Option<Msg>) -> Result<()> {
    loop {
        let mut stream = async_utils::repeat_until_ok(
            || connection::connect_to_server(ip, port, "User: "),
            None,
        )
        .await;
        if network::handle_connection(&mut stream, &mut lost_msg)
            .await
            .is_err()
        {
            continue;
        }
    }
}
