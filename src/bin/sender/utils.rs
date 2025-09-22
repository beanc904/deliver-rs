pub mod addr_cache;
pub mod args;

use clap::Parser;
use dialoguer::Input;

use addr_cache::AddrCache;
use args::Args;

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
    let mut cache = AddrCache::load();
    let ip_addr = match args.ip {
        Some(ip) => {
            // If an IP address is provided, use it
            let ip_addr = format!("{}:{}", ip, args.port);
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
                    let ip_addr = format!("{}:{}", ip, args.port);
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
use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};

/// Send a file to the specified IP address over TCP.
/// Displays a progress bar during the transfer.
/// # Arguments
/// * `sender_target` - A Path ref that holds the path of the file to be sent.
/// * `ip_addr` - A string slice that holds the IP address and port of the server.
/// # Returns
/// An `anyhow::Result<()>` indicating success or failure.
/// # Example
/// ```
/// tcp_sender(Path::new("path/to/file.txt"), "192.168.172.58:9000")?;
/// ```
pub fn tcp_sender(sender_target: &Path, ip_addr: &str) -> anyhow::Result<()> {
    let file_name = sender_target
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    // If the target is a directory (uzip), remove the .uzip extension for display
    let format_name = match sender_target.is_dir() {
        true => file_name.trim_end_matches(".uzip"),
        false => &file_name,
    };
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

    println!("Sent file: {} ({} bytes)", format_name, file_size);
    Ok(())
}
