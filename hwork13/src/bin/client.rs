use clap::Parser;
use hwork13::{parse_input, parse_socket_addr, send_message, MessageType};
use std::net::SocketAddr;
use std::net::TcpStream;
use std::thread;
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

fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    let server_addr = &config.address;
    let mut stream = match TcpStream::connect(server_addr) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to connect to {server_addr}: {e}");
            std::process::exit(1);
        }
    };

    thread::spawn({
        let Ok(stream) = stream.try_clone() else {
            error!("Failed to clone TCP stream.");
            std::process::exit(1);
        };
        move || {
            if let Err(e) = handle_server(stream) {
                error!("Error receiving from server: {:?}", e);
                std::process::exit(1);
            }
        }
    });

    loop {
        match parse_input() {
            Ok(msg) => {
                if let Err(e) = send_message(&mut stream, &msg) {
                    error!("Send message error:{e}");
                }
                if let MessageType::Quit = msg {
                    break;
                }
            }
            Err(e) => error!("Failed to parse input: {:?}", e),
        }
    }
}
