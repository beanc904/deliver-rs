use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Args {
    /// The file(include file and directory) to send
    #[arg(short, long)]
    pub file: String,

    /// The server IP address
    #[arg(short, long)]
    pub ip: Option<String>,

    /// The server port
    #[arg(short, long)]
    pub port: Option<u16>,
}
