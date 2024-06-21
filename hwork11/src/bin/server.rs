use clap::Parser;
use hwork11::{parse_socket_addr, ResponseType};
use std::collections::HashMap;
use std::error::Error;
use std::net::{SocketAddr, TcpListener};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use tracing::{error, info};

#[path = "../server_utils.rs"]
mod server_utils;
use crate::server_utils::{broadcast_response, handle_client, Clients};

/// Server configuration
#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value = "127.0.0.1:11111", value_parser = parse_socket_addr)]
    address: SocketAddr,
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    let config = Config::parse();
    let listener = TcpListener::bind(config.address)?;
    info!("Server running on {}", config.address);

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let (tx, rx): (Sender<ResponseType>, Receiver<ResponseType>) = mpsc::channel();

    // Thread for broadcasting messages to all clients
    std::thread::spawn({
        let clients = clients.clone();
        move || {
            for response in rx {
                broadcast_response(response, clients.clone());
            }
        }
    });

    for stream in listener.incoming() {
        let stream = stream?;
        let addr: SocketAddr = stream.peer_addr()?;
        let clients = clients.clone();
        let tx = tx.clone();

        clients.lock().unwrap().insert(addr, stream.try_clone()?);

        std::thread::spawn(move || {
            if let Err(e) = handle_client(stream, addr, clients, tx) {
                error!("Error handling client: {:?}", e);
            }
        });
    }

    Ok(())
}
