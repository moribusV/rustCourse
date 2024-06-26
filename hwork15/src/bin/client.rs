use anyhow::Result;
use clap::Parser;
use hwork15::{parse_input, parse_socket_addr, send_message, MessageType};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::error;

#[path = "../client_utils.rs"]
mod client_utils;
use client_utils::handle_server;

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

    let (reader, writer) = stream.into_split();
    let stream_writer_sync = Arc::new(Mutex::new(writer));
    let stream_writer_clone = stream_writer_sync.clone();

    let write_task = tokio::spawn(async move {
        let buf_read_lines = BufReader::new(stdin());
        let mut lines = buf_read_lines.lines();
        loop {
            match parse_input(&mut lines).await {
                Ok(msg) => {
                    let mut stream = stream_writer_clone.lock().await;
                    if let Err(e) = send_message(&mut *stream, &msg).await {
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
        let buf_read_lines = BufReader::new(stdin());
        let mut lines = buf_read_lines.lines();
        if let Err(e) = handle_server(reader, &stream_writer_sync.clone(), &mut lines).await {
            error!("Error receiving from server: {:?}", e);
        }
    });

    tokio::join!(write_task, read_task);

    Ok(())
}
