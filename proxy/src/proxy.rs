use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::Mutex,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{self, Duration},
};

use modules::{async_utils, network::connection};
pub struct Proxy {
    users: Mutex<HashMap<IpAddr, usize>>,
    connection_restrictions: usize,

    server_ip: String,
    server_port: u32,
}

impl Proxy {
    pub fn new(connection_restrictions: usize, server_ip: String, server_port: u32) -> Self {
        Self {
            users: Mutex::new(HashMap::<IpAddr, usize>::new()),
            connection_restrictions,

            server_ip,
            server_port,
        }
    }
}

impl Proxy {
    pub fn add_user(&self, user: IpAddr) {
        let mut map = self.users.lock().unwrap();
        let count = map.entry(user).or_insert(0);
        *count += 1;
    }

    pub fn del_user(&self, user: IpAddr) {
        let mut map = self.users.lock().unwrap();
        if let Some(connections) = map.remove(&user) {
            if connections != 1 {
                map.insert(user, connections - 1);
            }
        }
    }

    pub fn check_user(&self, user: IpAddr) -> bool {
        let map = self.users.lock().unwrap();
        if let Some(connections) = map.get(&user) {
            connections < &self.connection_restrictions
        } else {
            true
        }
    }
}

impl Proxy {
    const BUF_SIZE: usize = 8192;

    pub async fn handle_connection(
        &self,
        client_stream: TcpStream,
        client_address: SocketAddr,
        timeout_message: u64,
    ) {
        let user = format!("User: {}: ", client_address);

        if !self.check_user(client_address.ip()) {
            tracing::warn!("{user}limiting connected clients from one IP");
            return;
        }

        self.add_user(client_address.ip());
        tracing::info!("{user}added");

        let server_stream = async_utils::repeat_until_ok(
            || connection::connect_to_server(&self.server_ip, self.server_port, &user),
            None,
        );
        let (server_stream,) = tokio::join!(server_stream);

        let (client_reader, client_writer) = client_stream.into_split();
        let (server_reader, server_writer) = server_stream.into_split();

        let mut client_to_server = {
            let logs = ("{user}connection lost", "Server: timeout reading data");
            tokio::spawn(async move {
                Self::client_or_server(
                    client_reader,
                    server_writer,
                    logs,
                    Duration::from_secs(timeout_message),
                )
                .await;
            })
        };

        let mut server_to_client = {
            let logs = ("Server: connection lost", "{user}timeout reading data");
            tokio::spawn(async move {
                Self::client_or_server(
                    server_reader,
                    client_writer,
                    logs,
                    Duration::from_secs(timeout_message),
                )
                .await;
            })
        };

        tokio::select! {
            _ = &mut client_to_server => (),
            _ = &mut server_to_client => (),
        }

        client_to_server.abort();
        server_to_client.abort();

        tracing::warn!("{user}connection refused");
        self.del_user(client_address.ip());
    }

    async fn client_or_server<Reader, Writer>(
        mut reader: Reader,
        mut writer: Writer,

        logs: (&str, &str),
        timeout_message: Duration,
    ) where
        Reader: AsyncReadExt + Unpin,
        Writer: AsyncWriteExt + Unpin,
    {
        loop {
            let mut buf = [0; Self::BUF_SIZE];

            match time::timeout(timeout_message, reader.read(&mut buf)).await {
                Ok(Ok(count)) if count == 0 => {
                    tracing::warn!("{}", logs.0);
                    return;
                }
                Ok(Ok(count)) => {
                    if writer.write_all(&buf[..count]).await.is_err() {
                        return;
                    }
                }
                Err(_) => {
                    tracing::warn!("{}", logs.1);
                    return;
                }
                _ => return,
            }
        }
    }
}
