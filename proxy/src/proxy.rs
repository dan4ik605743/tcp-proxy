use std::{
    collections::{hash_map::Entry, HashMap},
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
    fn add_user(&self, user: IpAddr) {
        self.users
            .lock()
            .unwrap()
            .entry(user)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    fn del_user(&self, user: IpAddr) {
        let mut map = self.users.lock().unwrap();

        if let Entry::Occupied(user) = map.entry(user).and_modify(|connections| *connections -= 1) {
            if user.get() == &0 {
                user.remove_entry();
            }
        }
    }

    fn check_user(&self, user: IpAddr) -> bool {
        if let Some(connections) = self.users.lock().unwrap().get(&user) {
            return connections < &self.connection_restrictions;
        }
        true
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

        let logs = (
            format!("{user}connection lost"),
            "Server: timeout reading data".to_owned(),
            "Server: connection lost".to_owned(),
            format!("{user}timeout reading data"),
        );

        let mut client_to_server = tokio::spawn(async move {
            Self::transfer_data(
                client_reader,
                server_writer,
                (logs.0, logs.1),
                Duration::from_secs(timeout_message),
            )
            .await
        });

        let mut server_to_client = tokio::spawn(async move {
            Self::transfer_data(
                server_reader,
                client_writer,
                (logs.2, logs.3),
                Duration::from_secs(timeout_message),
            )
            .await
        });

        tokio::select! {
            _ = &mut client_to_server => (),
            _ = &mut server_to_client => (),
        }

        client_to_server.abort();
        server_to_client.abort();

        tracing::warn!("{user}connection refused");
        self.del_user(client_address.ip());
    }

    async fn transfer_data<Reader, Writer>(
        mut reader: Reader,
        mut writer: Writer,

        logs: (String, String),
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

#[cfg(test)]
mod tests;
