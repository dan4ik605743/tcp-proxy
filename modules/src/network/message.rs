use anyhow::Result;
use serde::{Deserialize, Serialize};

use bytes::{Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, ErrorKind};

#[derive(Serialize, Deserialize, Default)]
pub struct Message(pub Msg);
pub type Msg = String;
const MESSAGE_SIZE: usize = 10000000;

pub async fn send_message<Stream>(stream: &mut Stream, message: &Message) -> Result<()>
where
    Stream: AsyncWriteExt + Unpin,
{
    let bytes: Bytes = serde_json::to_vec(&message)?.into();
    // let bytes = serde_json::to_vec(&message)?;
    stream.write_u64(bytes.len() as u64).await?;
    stream.write_all(&bytes[..]).await?;
    Ok(())
}

#[allow(clippy::uninit_vec)]
pub async fn read_message<Stream>(stream: &mut Stream) -> tokio::io::Result<Message>
where
    Stream: AsyncReadExt + Unpin,
{
    let message_size = stream.read_u64().await? as usize;
    if message_size > 0 && message_size <= MESSAGE_SIZE {
        let mut buf = BytesMut::with_capacity(message_size);
        // let mut buf = Vec::<u8>::with_capacity(message_size);
        unsafe {
            buf.set_len(message_size);
        }
        stream.read_exact(&mut buf[..]).await?;
        Ok(serde_json::from_slice(&buf[..])?)
    } else {
        Err(tokio::io::Error::new(
            ErrorKind::Other,
            "Message size is larger than the limit",
        ))
    }
}
