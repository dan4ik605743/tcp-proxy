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

    /// CONFIG
    #[arg(short, long)]
    pub config: Option<String>,
}
