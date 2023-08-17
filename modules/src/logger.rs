use time::macros::{format_description, offset};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logger() {
    // Env
    let env_filter = EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();

    // Timer
    let offset = offset!(+7);
    let time_format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
    let timer = fmt::time::OffsetTime::new(offset, time_format);

    // Init
    tracing_subscriber::registry()
        .with(fmt::layer().with_timer(timer).with_target(false))
        .with(env_filter)
        .init();
}
