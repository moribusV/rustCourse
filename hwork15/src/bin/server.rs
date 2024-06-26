use anyhow::{Context, Result};
use clap::Parser;
use hwork15::{parse_socket_addr, send_message};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::broadcast;
use tokio::{net::TcpListener, sync::Mutex};
use tracing::{error, info};

#[path = "../server_utils.rs"]
mod server_utils;
use crate::server_utils::handle_client;

#[path = "../db.rs"]
mod db; // Ensure this line is added to import db.rs
use db::Database;

/// Server configuration
#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value = "127.0.0.1:11111", value_parser = parse_socket_addr)]
    address: SocketAddr,
    #[arg(short, long, default_value = "sqlite:./db.sqlite")]
    database_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    let config = Config::parse();
    let addr = &config.address;
    let database_url = &config.database_url;

    let database = Database::new(database_url).await?;
    let database = Arc::new(database);

    let listener = TcpListener::bind(addr)
        .await
        .context("Failed to bind to socket")?;

    info!("Server running on {}", addr);

    let (br_send, _br_recv) = broadcast::channel(1024);

    loop {
        let Ok((stream, addr)) = listener.accept().await else {
            error!("Failed to accept connection");
            continue;
        };

        info!("New connection from {}", addr);

        let sender = br_send.clone();
        let mut receiver = br_send.subscribe();

        let (stream_reader, stream_writer) = stream.into_split();

        let stream_writer_sync = Arc::new(Mutex::new(stream_writer));
        let stream_writer_clone = stream_writer_sync.clone();

        let db_clone = Arc::clone(&database);

        tokio::spawn(async move {
            if let Err(e) = handle_client(
                stream_reader,
                &stream_writer_sync.clone(),
                addr,
                sender,
                db_clone,
            )
            .await
            {
                error!("Error handling client: {:?}", e);
            }
        });

        tokio::spawn(async move {
            while let Ok((msg, other_addr)) = receiver.recv().await {
                if other_addr == addr {
                    continue;
                }
                let mut stream = stream_writer_clone.lock().await;
                if let Err(e) = send_message(&mut *stream, &msg).await {
                    error!("{e}");
                    break;
                }
                drop(stream);
            }
        });
    }
}
