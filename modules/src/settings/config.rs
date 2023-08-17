use std::fs;

use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Config: Serialize + DeserializeOwned {
    fn get_config_path(&self) -> &str;
}

#[derive(Serialize, Deserialize)]
pub struct ClientOrServer {
    pub ip: String,
    pub port: u32,
}

impl ClientOrServer {
    const DEFAULT_CLIENT_OR_SERVER_CONFIG_PATH: &str = ".settings.json";

    pub fn new(ip: String, port: u32) -> Self {
        Self { ip, port }
    }
}

impl Config for ClientOrServer {
    fn get_config_path(&self) -> &str {
        Self::DEFAULT_CLIENT_OR_SERVER_CONFIG_PATH
    }
}

#[derive(Serialize, Deserialize)]
pub struct Proxy {
    pub ip: String,
    pub port: u32,

    pub server_ip: String,
    pub server_port: u32,

    pub connection_restrictions: usize,
    pub timeout_message: u64,
}

impl Proxy {
    const DEFAULT_PROXY_CONFIG_PATH: &str = ".proxy_settings.json";

    pub fn new(
        ip: String,
        port: u32,
        server_ip: String,
        server_port: u32,
        connection_restrictions: usize,
        timeout_message: u64,
    ) -> Self {
        Self {
            ip,
            port,
            server_ip,
            server_port,
            connection_restrictions,
            timeout_message,
        }
    }
}

impl Config for Proxy {
    fn get_config_path(&self) -> &str {
        Self::DEFAULT_PROXY_CONFIG_PATH
    }
}

pub fn init_config<T: Config>(config: T, config_path: Option<String>) -> Result<T> {
    let read_config = |config_path: String, config| match self::read_config::<T>(&config_path) {
        Ok(read_config) => {
            tracing::info!("Config read successfully");
            Ok(read_config)
        }
        Err(_) => {
            tracing::warn!("Config read failed");
            tracing::info!("Overwriting config from terminal options and use it");
            write_config(&config, &config_path)?;
            Ok(config)
        }
    };

    if let Some(config_path) = config_path {
        tracing::info!("Reading config by custom path...");
        read_config(config_path, config)
    } else if fs::File::open(config.get_config_path()).is_ok() {
        tracing::info!("Reading config by default path...");
        read_config(config.get_config_path().to_string(), config)
    } else {
        tracing::warn!("Config not found");
        tracing::info!("Creating a config on the default path from terminal options and use it");
        write_config(&config, config.get_config_path())?;
        Ok(config)
    }
}

fn write_config<'a, T: Config>(config: &'a T, config_path: &'a str) -> Result<&'a T> {
    let data = serde_json::to_string(&config)?;
    fs::write(config_path, data)?;
    Ok(config)
}

fn read_config<T: Config>(config_path: &str) -> Result<T> {
    let settings: T = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    Ok(settings)
}
