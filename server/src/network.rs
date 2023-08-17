use std::net::SocketAddr;

use tokio::{io::ErrorKind, net::TcpStream};

use modules::network::message::{self, Message};

pub async fn handle_connection(mut stream: TcpStream, address: SocketAddr) {
    let user = format!("User: {}: ", address);
    tracing::info!("{user}connected");

    loop {
        match message::read_message(&mut stream).await {
            Ok(msg) => {
                tracing::info!("{user}data received: '{}'", msg.0);

                tracing::info!("{user}sending data...");
                match message::send_message(&mut stream, &Message("OK".to_owned())).await {
                    Ok(_) => tracing::info!("{user}successfully sending data"),
                    Err(_) => tracing::warn!("{user}failed sending data"),
                }
            }
            Err(err) => {
                if let ErrorKind::UnexpectedEof = err.kind() {
                    tracing::warn!("{user}disconnected");
                } else {
                    tracing::warn!("{user}{err}");
                }
                break;
            }
        }
    }
}
