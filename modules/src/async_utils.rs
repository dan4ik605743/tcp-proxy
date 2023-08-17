use std::{fmt::Display, future::Future, result::Result};

use tokio::time::{sleep, Duration};

pub async fn repeat_until_ok<T, E, F, Fut>(f: F, delay: Option<Duration>) -> T
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,

    E: Display,
{
    loop {
        match f().await {
            Ok(val) => {
                break val;
            }
            Err(err) => {
                tracing::warn!("{err}");
                sleep(delay.unwrap_or(Duration::from_secs(5))).await;
            }
        }
    }
}
