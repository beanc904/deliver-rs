use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Args {
    /// The file to send
    #[arg(short, long)]
    pub file: String,

    /// The server IP address
    #[arg(short, long)]
    pub ip: Option<String>,

    /// The server port
    #[arg(short, long, default_value_t = 9000)]
    pub port: u16,
}
