use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// IP
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub ip: String,

    /// PORT
    #[arg(short, long, default_value_t = 7878)]
    pub port: u32,

    /// SERVER_IP
    #[arg(short = 'I', long, default_value_t = String::from("127.0.0.1"))]
    pub server_ip: String,

    /// SERVER_PORT
    #[arg(short = 'P', long, default_value_t = 7879)]
    pub server_port: u32,

    /// CONNECTION RESTRICTIONS
    #[arg(short = 'C', long, default_value_t = 50)]
    pub connection_restrictions: usize,

    /// TIMEOUT_MESSAGE_SECS
    #[arg(short, long, default_value_t = 50)]
    pub timeout_message: u64,

    /// CONFIG
    #[arg(short, long)]
    pub config: Option<String>,
}
