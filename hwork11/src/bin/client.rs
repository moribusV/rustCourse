use clap::Parser;
use std::error::Error;
use std::net::TcpStream;
use std::thread;

#[path = "../client_utils.rs"]
mod client_utils;
use client_utils::handle_server;

#[path = "../shared.rs"]
mod shared;
use crate::shared::{parse_inputt, send_message, MessageType};

/// Client configuration
#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value = "127.0.0.1:11111")]
    address: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let config = Config::parse();
    let server_addr = &config.address;
    let mut stream = TcpStream::connect(server_addr)?;

    thread::spawn({
        let stream = stream.try_clone()?;
        move || {
            if let Err(e) = handle_server(stream) {
                eprintln!("Error receiving from server: {:?}", e);
            }
        }
    });

    loop {
        match parse_inputt() {
            Ok(msg) => {
                send_message(&mut stream, &msg)?;
                if let MessageType::Quit = msg {
                    break;
                }
            }
            Err(e) => eprintln!("Failed to parse input: {:?}", e),
        }
    }

    Ok(())
}
