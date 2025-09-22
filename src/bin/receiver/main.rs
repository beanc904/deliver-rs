//! This is the server.
//! It listens for incoming connections, receives a file, and verifies its integrity.

mod utils;

use clap::Parser;
use console::style;

use crate::utils::{show_ipv4, tcp_listener};

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
struct Args {
    /// The port to listen on
    #[arg(short, long, default_value_t = 9000)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ANCHOR: some init events
    env_logger::init();
    let args = Args::parse();
    // ANCHOR_END: some init events

    println!("{}", style("Starting server...".to_string()).bold().blue());

    // ANCHOR: cfg info
    let args_port = args.port;
    let ip_addr = format!("0.0.0.0:{}", args_port);
    // ANCHOR_END: cfg info

    show_ipv4();

    println!(
        "Server listening on port {}... (press 'q' to quit)",
        style(args_port).bold().green()
    );

    tcp_listener(&ip_addr).await
}
