use anyhow::{Context, Result};
use hwork13::{receive_message, send_message, MessageType, ResponseType};
use std::collections::HashMap;
use std::io::Read;
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use tracing::{error, info};

pub type Clients = Arc<Mutex<HashMap<SocketAddr, TcpStream>>>;

pub fn handle_client(
    mut stream: TcpStream,
    addr: SocketAddr,
    clients: Clients,
    tx: Sender<ResponseType>,
) -> Result<()> {
    loop {
        let cli_message = match receive_message::<MessageType>(&mut stream) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Error receiving message from {}: {:?}", addr, e);
                break;
            }
        };

        let res = match cli_message {
            MessageType::File(path) => handle_file(&path).unwrap_or_else(|e| {
                ResponseType::Error(format!("Error handling file{}: {}", path, e))
            }),
            MessageType::Image(path) => handle_image(&path).unwrap_or_else(|e| {
                ResponseType::Error(format!("Error handling image {}: {}", path, e))
            }),
            MessageType::Text(text) => ResponseType::Text(format!("{}: {}", addr, text)),
            MessageType::Quit => {
                clients.lock().unwrap().remove(&addr);
                info!("Client {} has disconnected.", addr);
                stream.shutdown(Shutdown::Both)?;
                return Ok(());
            }
        };

        tx.send(res).context("Failed to send response")?;
    }

    clients.lock().unwrap().remove(&addr);
    Ok(())
}

pub fn broadcast_response(response: ResponseType, clients: Clients) {
    let clients = clients.lock().unwrap();

    for (addr, client) in clients.iter() {
        let mut client = client.try_clone().expect("Failed to clone TcpStream");
        if let Err(e) = send_message(&mut client, &response) {
            error!("Failed to send response to {}: {:?}", addr, e);
        }
    }
}

fn handle_file(path: &str) -> Result<ResponseType> {
    let file_name = get_file_name(path).context("Failed to get file name")?;
    let contents = read_file(path).context("Failed to read file IAMHERE")?;
    Ok(ResponseType::File(file_name, contents))
}

fn handle_image(path: &str) -> Result<ResponseType> {
    let file_name = get_file_name(path).context("Failed to get image name")?;
    let contents = read_file(path).context("Failed to read image")?;
    Ok(ResponseType::Image(file_name, contents))
}

fn read_file(path: &str) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(path).context("Failed to open file")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .context("Failed to read file")?;
    Ok(contents)
}

fn get_file_name(path: &str) -> Result<String> {
    let name = std::path::Path::new(path)
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
        .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

    Ok(name)
}
