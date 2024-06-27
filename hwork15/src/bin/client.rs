use anyhow::Result;
use clap::Parser;
use hwork15::{parse_input, parse_socket_addr, send_message, MessageType};
use std::net::SocketAddr;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tracing::{error, info};

#[path = "../client_utils.rs"]
mod client_utils;
use client_utils::{handle_authentication_or_registration, handle_server};

/// Client configuration
#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value = "127.0.0.1:11111", value_parser = parse_socket_addr)]
    address: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    let server_addr = &config.address;
    let stream = TcpStream::connect(server_addr).await?;

    let (mut reader, mut writer) = stream.into_split();

    let buf_read_lines = BufReader::new(stdin());
    let mut lines = buf_read_lines.lines();
    handle_authentication_or_registration(&mut reader, &mut writer, &mut lines).await?;
    info!("Authentication successful. I was waiting on you.. Neo.");

    let write_task = tokio::spawn(async move {
        loop {
            match parse_input(&mut lines).await {
                Ok(msg) => {
                    if let Err(e) = send_message(&mut writer, &msg).await {
                        error!("Send message error: {:?}", e);
                    }
                    if let MessageType::Quit = msg {
                        break;
                    }
                }
                Err(e) => error!("Failed to parse input: {:?}", e),
            }
        }
    });

    let read_task = tokio::spawn(async move {
        if let Err(e) = handle_server(&mut reader).await {
            error!("Error receiving from server: {:?}", e);
        }
    });

    tokio::join!(write_task, read_task);

    Ok(())
}
