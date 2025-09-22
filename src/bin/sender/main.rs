//! This is the client.
//! It connects to a server, sends a file, and displays a progress bar.

pub mod utils;

use clap::Parser;

use crate::utils::args::Args;
use crate::utils::get_addr_from_cache;
use crate::utils::tcp_sender;

use deliver::pkg_info::PkgInfo;
use std::path::Path;
use zip_extensions::*;

fn main() -> anyhow::Result<()> {
    // ANCHOR: some init events
    env_logger::init();
    let args = Args::parse();
    // ANCHOR_END: some init events

    // ANCHOR: cfg info
    let args_file = Path::new(&args.file);
    let ip_addr = get_addr_from_cache();
    // ANCHOR_END: cfg info

    // ANCHOR: judge file_path is file or dir
    if args_file.is_file() {
        // It's a file.
        println!("Sending file: {:?} to {}", args_file, ip_addr);
        tcp_sender(&args_file, &ip_addr)
    } else if args_file.is_dir() {
        // It's a directory.

        // Get the directory name to create a uzip file with the same name.
        let dir_name = args_file
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Failed to get directory name."))?
            .to_string_lossy();
        // Create a temporary uzip file in the cache directory.
        let archive_path = PkgInfo::new()
            .get_cache_dir()
            .join(dir_name.to_string())
            .with_extension("uzip");
        let source_path = args_file.to_path_buf();
        zip_create_from_directory(&archive_path, &source_path)?;
        log::info!(
            "Created archive from {:?} at {:?}",
            source_path,
            archive_path
        );

        println!("Sending directory: {:?} to {}", args_file, ip_addr);
        tcp_sender(Path::new(archive_path.to_str().unwrap()), &ip_addr)?;

        // Clean up the temporary archive file after sending.
        std::fs::remove_file(&archive_path)?;
        anyhow::Ok(())
    } else {
        return Err(anyhow::anyhow!(
            "The path is neither a file nor a directory."
        ));
    }
    // ANCHOR_END: judge file_path is file or dir
}
