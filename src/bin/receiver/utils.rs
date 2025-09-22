/// It will show the server's IPv4 address.
/// Including vurtual interfaces and physical interfaces.
pub fn show_ipv4() {
    // ANCHOR: show the server's IP address
    match if_addrs::get_if_addrs() {
        Ok(interfaces) => {
            for interface in interfaces {
                // Filter out loopback interfaces and non-IPv4 addresses
                if !interface.is_loopback() {
                    match interface.addr {
                        if_addrs::IfAddr::V4(_) => {
                            println!(
                                "Server IP: {} - Interface: {}",
                                interface.addr.ip(),
                                interface.name
                            );
                        }
                        _ => continue, // Only handle IPv4 addresses
                    }
                }
            }
        }
        Err(e) => log::error!("Error retrieving interfaces: {}", e),
    }
    // ANCHOR_END: show the server's IP address
}

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tokio::time::{Duration, sleep};

/// TCP listener that handles incoming connections and allows quitting with 'q'
/// It will save the file in the work dir.
/// # Arguments
/// - `ip_addr`: A string slice that holds the IP address and port of the server.
/// # Returns
/// An `anyhow::Result<()>` indicating success or failure.
pub async fn tcp_listener(ip_addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(ip_addr).await?;

    // Set raw mode for stdin to capture 'q' key press
    enable_raw_mode()?;

    loop {
        tokio::select! {
            // Accept incoming connections
            connect = listener.accept() => {
                match connect {
                    Ok((stream, addr)) => {
                        tokio::spawn(async move {
                            if let Err(e) = handle_client(stream, addr).await {
                                log::error!("Error handling client {}: {}", addr, e);
                            }
                        });
                    }
                    Err(e) => log::error!("Failed to accept connection: {}", e),
                }
            }

            // Check for 'q' key press to quit
            res = tokio::task::spawn_blocking(|| {
                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let Event::Key(key_event) = event::read().unwrap() {
                        if key_event.code == KeyCode::Char('q') {
                            return true;
                        }
                    }
                }
                false
            }) => {
                match res {
                    Ok(true) => {
                        let res = "Shutting down server...".to_string();
                        println!("{}\r", style(res).bold().blue());
                        break;
                    }
                    Ok(false) => {}
                    Err(e) => log::error!("Error reading key event: {}", e),
                }
            }
        }

        // Sleep to prevent busy waiting
        sleep(Duration::from_millis(100)).await;
    }

    disable_raw_mode()?;
    Ok(())
}

use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::path::PathBuf;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use zip_extensions::*;

async fn handle_client(mut stream: TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
    println!("Client connected: {}\r", addr);

    // ANCHOR: receive file name length, name, size, and checksum
    let mut len_buf = [0u8; 2];
    stream.read_exact(&mut len_buf).await?;
    let name_len = u16::from_be_bytes(len_buf) as usize;

    let mut name_buf = vec![0u8; name_len];
    stream.read_exact(&mut name_buf).await?;
    let file_name = String::from_utf8(name_buf).unwrap();

    let mut size_buf = [0u8; 8];
    stream.read_exact(&mut size_buf).await?;
    let file_size = u64::from_be_bytes(size_buf);

    let mut checksum_buf = [0u8; 32];
    stream.read_exact(&mut checksum_buf).await?;
    // ANCHOR_END: receive file name length, name, size, and checksum

    // ANCHOR: display file info
    let (format_name, file_type) = match file_name.ends_with(".uzip") {
        true => (file_name.trim_end_matches(".uzip"), "Directory"),
        false => (file_name.as_str(), "File"),
    };
    println!("Receiving dir: {} ({} bytes)\r", format_name, file_size);
    // ANCHOR_END: display file info

    // ANCHOR: receive file content with progress bar
    let mut file = File::create(&file_name)?;
    let mut received: u64 = 0;
    let mut buffer = [0; 8192];

    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} {wide_bar} {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message(format!("Receiving {}", format_name));

    let mut hasher = Sha256::new();

    while received < file_size {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        hasher.update(&buffer[..n]);
        received += n as u64;
        pb.set_position(received);
    }

    pb.finish_with_message("Receive complete");
    // ANCHOR_END: receive file content with progress bar

    // ANCHOR: verify checksum and cleanup
    let calculated_checksum = hasher.finalize();
    if calculated_checksum.as_slice() == checksum_buf {
        let res = format!(
            "{} {} received successfully. Checksum OK.",
            file_type, format_name
        );
        println!("{}\r", style(res).green());

        if file_type == "Directory" {
            // Unzip the received .uzip file
            let archive_file = PathBuf::from(&file_name);
            let target_dir = PathBuf::from(format_name);
            zip_extract(&archive_file, &target_dir)?;

            log::info!(
                "Extracted archive {} to directory {}",
                file_name,
                format_name
            );

            // Remove the .uzip file after extraction
            std::fs::remove_file(&file_name)?;
        }
    } else {
        println!(
            "{} {} received, but checksum mismatch!\r",
            file_type, format_name
        );
    }
    // ANCHOR_END: verify checksum and cleanup

    Ok(())
}
