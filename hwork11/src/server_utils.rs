use log::{error, info};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::shared::{receive_message, send_message, MessageType, ResponseType};

pub type Clients = Arc<Mutex<HashMap<SocketAddr, TcpStream>>>;

pub fn handle_client(
    stream: TcpStream,
    addr: SocketAddr,
    clients: Clients,
    tx: Sender<ResponseType>,
) -> Result<(), Box<dyn Error>> {
    let mut stream = stream;
    loop {
        let cli_message = match receive_message::<MessageType>(&mut stream) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Error receiving message from {}: {:?}", addr, e);
                break;
            }
        };

        let res = match cli_message {
            MessageType::File(path) => handle_file(&path)?,
            MessageType::Image(path) => handle_image(&path)?,
            MessageType::Text(text) => ResponseType::Text(format!("{}: {}", addr, text)),
            MessageType::Quit => {
                // Remove the client from the list
                clients.lock().unwrap().remove(&addr);
                info!("Client {} has disconnected.", addr);
                stream.shutdown(Shutdown::Both)?;
                return Ok(());
            }
        };

        println!("before send msg");
        tx.send(res).unwrap();
    }

    clients.lock().unwrap().remove(&addr);
    Ok(())
}

/// Broadcasts a response message to all connected clients.
pub fn broadcast_response(response: ResponseType, clients: Clients) {
    let clients = clients.lock().unwrap();
    for (addr, client) in clients.iter() {
        let mut client = client.try_clone().expect("Failed to clone TcpStream");
        if let Err(e) = send_message(&mut client, &response) {
            error!("Failed to send response to {}: {:?}", addr, e);
        }
    }
}

/// Reads a file from the given path and returns its contents.
fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

/// Extracts the file name from the given path.
fn get_file_name(path: &str) -> Result<String, Box<dyn Error>> {
    let name = Path::new(path)
        .file_name()
        .and_then(OsStr::to_str)
        .map(String::from)
        .ok_or_else(|| "Invalid file path".to_string())?;

    Ok(name)
}

/// Handles the file reading and returns a response containing the file or an error message.
fn handle_file(path: &str) -> Result<ResponseType, Box<dyn Error>> {
    let file_name = get_file_name(path)?;

    let res = match read_file(path) {
        Ok(contents) => ResponseType::File(file_name, contents),
        Err(e) => {
            println!("Error reading file: {}", e);
            ResponseType::Text(String::from("Error reading file"))
        }
    };

    Ok(res)
}

/// Handles the image reading and returns a response containing the image or an error message.
fn handle_image(path: &str) -> Result<ResponseType, Box<dyn Error>> {
    let file_name = get_file_name(path)?;

    let res = match read_file(path) {
        Ok(contents) => ResponseType::Image(file_name, contents),
        Err(e) => {
            println!("Error reading image: {}", e);
            ResponseType::Text(String::from("Error reading image"))
        }
    };

    Ok(res)
}
