pub mod addr_cache;
pub mod args;

use clap::Parser;
use dialoguer::Input;

use addr_cache::AddrCache;
use args::Args;
use deliver::cfg::Cfg;

/// Get the IP address from command line arguments or cache.
/// If not provided, prompt the user for input.
/// # Returns
/// A `String` representing the IP address and port in the format "{ip}:{port}".
///
/// # Example
/// ```
/// let ip_addr = get_addr_from_cache();
/// ```
pub fn get_addr_from_cache() -> String {
    let args = Args::parse();
    let port = args.port.unwrap_or_else(|| {
        let cfg = Cfg::load();
        cfg.get_port()
    });
    let mut cache = AddrCache::load();
    let ip_addr = match args.ip {
        Some(ip) => {
            // If an IP address is provided, use it
            let ip_addr = format!("{}:{}", ip, port);
            cache.add_addr(ip_addr.clone());
            ip_addr
        }
        None => {
            // If no IP address is provided in cli, first read from history file
            match cache.select_addr() {
                Some(ip_addr) => ip_addr,
                None => {
                    // If history file does not exist or is empty, prompt user for IP address
                    let ip: String = Input::new()
                        .with_prompt("Enter server IP address")
                        .interact_text()
                        .unwrap();
                    let ip_addr = format!("{}:{}", ip, port);
                    cache.add_addr(ip_addr.clone());
                    ip_addr
                }
            }
        }
    };
    cache.save();
    ip_addr
}

use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use zip_extensions::*;

use deliver::pkg_info::PkgInfo;

/// Send a file to the specified IP address over TCP.
/// Displays a progress bar during the transfer.
/// # Arguments
/// * `sender_target` - A Path ref that holds the path of the file/dir to be sent.
/// * `ip_addr` - A string slice that holds the IP address and port of the server.
/// # Returns
/// An `anyhow::Result<()>` indicating success or failure.
/// # Example
/// ```
/// tcp_sender(Path::new("path/to/file.txt"), "192.168.172.58:9000")?;
/// ```
pub fn tcp_sender(sender_target: &Path, ip_addr: &str) -> anyhow::Result<()> {
    // ANCHOR: judge the sender_target is file or dir
    let file_type;
    let sender_target: PathBuf = if !sender_target.exists() {
        return Err(anyhow::anyhow!(
            "The specified path does not exist: {:?}",
            sender_target
        ));
    } else {
        if sender_target.is_file() {
            // It's a file.
            println!("Sending file: {:?} to {}", sender_target, ip_addr);
            file_type = "file";
            sender_target.to_owned()
        } else if sender_target.is_dir() {
            // It's a directory.
            // Get the directory name to create a uzip file with the same name in cache.
            let dir_name = sender_target
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Failed to get directory name."))?
                .to_string_lossy();
            // Create a temporary uzip file in the cache directory.
            let archive_path = PkgInfo::new()
                .get_cache_dir()
                .join(dir_name.to_string())
                .with_extension("uzip");
            let source_path = sender_target.to_path_buf();
            zip_create_from_directory(&archive_path, &source_path)?;
            log::debug!(
                "Created archive from {:?} at {:?}",
                source_path,
                archive_path
            );

            println!("Sending directory: {:?} to {}", sender_target, ip_addr);

            file_type = "directory";
            archive_path.to_owned()
        } else {
            return Err(anyhow::anyhow!(
                "The path is neither a file nor a directory."
            ));
        }
    };

    let file_name = sender_target
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    // If the target is a directory (uzip), remove the .uzip extension for display
    let format_name = match file_type {
        "directory" => file_name.trim_end_matches(".uzip"),
        _ => &file_name,
    };
    // ANCHOR_END: judge the sender_target is file or dir

    let mut file = File::open(&sender_target)?;
    let file_size = file.metadata()?.len();

    // ANCHOR: calculate SHA256 of the file
    let mut hasher = Sha256::new();
    let mut file_copy = File::open(&sender_target)?;
    let mut buf = [0u8; 8192];
    loop {
        let n = file_copy.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let checksum = hasher.finalize();
    // ANCHOR_END: calculate SHA256 of the file

    let mut stream = TcpStream::connect(ip_addr)?;

    // ANCHOR: send file name length, name, size, and checksum
    let name_len = file_name.len() as u16;
    stream.write_all(&name_len.to_be_bytes())?;

    stream.write_all(file_name.as_bytes())?;

    stream.write_all(&file_size.to_be_bytes())?;

    stream.write_all(&checksum)?;
    // ANCHOR_END: send file name length, name, size, and checksum

    // ANCHOR: send file content with progress bar
    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} {wide_bar} {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message(format!("Sending {}", format_name));
    // ANCHOR_END: send file content with progress bar

    // ANCHOR: send file content
    let mut sent: u64 = 0;
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        stream.write_all(&buf[..n])?;
        sent += n as u64;
        pb.set_position(sent);
    }
    pb.finish_with_message("Send complete");
    // ANCHOR_END: send file content

    println!("Sent {}: {} ({} bytes)", file_type, format_name, file_size);
    Ok(())
}
