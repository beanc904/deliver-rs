//! This is the client.
//! It connects to a server, sends a file, and displays a progress bar.

pub mod utils;

use clap::Parser;

use crate::utils::args::Args;
use crate::utils::get_addr_from_cache;
use crate::utils::tcp_sender;

use std::path::Path;

fn main() -> anyhow::Result<()> {
    // ANCHOR: some init events
    env_logger::init();
    let args = Args::parse();
    // ANCHOR_END: some init events

    // ANCHOR: cfg info
    let args_file = Path::new(&args.file);
    let ip_addr = get_addr_from_cache();
    // ANCHOR_END: cfg info

    tcp_sender(args_file, &ip_addr)
}
