use anyhow::{anyhow, Result};

use tokio::net::TcpStream;

pub async fn connect_to_server(ip: &str, port: u32, format: &str) -> Result<TcpStream> {
    tracing::info!("{format}connecting to server...");
    match TcpStream::connect(format!("{}:{}", ip, port)).await {
        Ok(stream) => {
            tracing::info!("{format}successful connection");
            Ok(stream)
        }
        Err(err) => {
            tracing::info!("{format}reconnecting...");
            Err(anyhow!("{format}{err}"))
        }
    }
}
