use anyhow::{bail, Result};

use tokio::net::TcpStream;

use modules::network::message::{self, Message, Msg};

pub async fn handle_connection(stream: &mut TcpStream, lost_msg: &mut Option<Msg>) -> Result<()> {
    loop {
        let msg = if lost_msg.is_some() {
            lost_msg.take().unwrap()
        } else {
            tools::getting_data()
        };

        if send_message(stream, msg.clone()).await.is_err() {
            lost_msg.replace(msg);
            bail!("");
        } else if let Ok(msg) = message::read_message(stream).await {
            tracing::info!("Server: message '{}'", msg.0);
        }
    }
}

async fn send_message(mut stream: &mut TcpStream, msg: Msg) -> Result<()> {
    tracing::info!("User: sending data to the server...");

    match message::send_message(&mut stream, &Message(msg)).await {
        Ok(_) => {
            tracing::info!("User: data sent successfully");

            Ok(())
        }
        Err(_) => {
            tracing::warn!("User: disconnected with server");
            tracing::info!("User: reconnecting...");

            bail!("");
        }
    }
}

mod tools {
    pub fn getting_data() -> String {
        tracing::info!("Data entry pending...");

        let mut data = String::new();
        std::io::stdin()
            .read_line(&mut data)
            .expect("Failed to read stdin");
        data.trim_end().to_string()
    }
}
