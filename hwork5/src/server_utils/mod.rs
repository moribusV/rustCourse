use bincode::{deserialize, serialize};

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::{
    error::Error,
    ffi::OsStr,
    fs::File,
    net::{Ipv4Addr, TcpListener, TcpStream},
    path::Path,
};
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    File(String),
    Image(String),
    Text,
    Quit,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    File(String, Vec<u8>),
    Image(String, Vec<u8>),
    Text(String),
    Quit(String),
}

fn serialize_message(message: &ResponseType) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(serialize(&message)?)
}

fn deserialize_message(data: &[u8]) -> Result<MessageType, Box<dyn Error>> {
    Ok(deserialize(data)?)
}

pub fn send_message(stream: &mut TcpStream, message: &ResponseType) -> Result<(), Box<dyn Error>> {
    let serialized = serialize_message(message)?;

    // Send the length of the serialized message (as 4-byte value).
    let len = serialized.len() as u32;
    stream.write_all(&len.to_be_bytes())?;

    // Send the serialized message.
    stream.write_all(&serialized)?;
    Ok(())
}

fn receive_message(stream: &mut TcpStream) -> Result<MessageType, Box<dyn Error>> {
    let mut len_buf = [0u8; 4];

    stream.read_exact(&mut len_buf)?;
    let exac_len = u32::from_be_bytes(len_buf) as usize;

    let mut message_buf = vec![0u8; exac_len];
    stream.read_exact(&mut message_buf)?;

    deserialize_message(&message_buf)
}

fn parse_socket_addr(socket: &[String]) -> String {
    if socket.len() == 3 {
        let socket_addr = socket[1..].join(":");

        if socket_addr.parse::<Ipv4Addr>().is_ok() {
            return socket_addr;
        }
    }
    "localhost:11111".to_string()
}
pub fn create_tcp_listener(socket: &[String]) -> Result<TcpListener, Box<dyn Error>> {
    let addr = parse_socket_addr(socket);
    let listener = TcpListener::bind(&addr)?;
    println!("Server listening on: {}", addr);
    Ok(listener)
}

pub fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        let cli_message = receive_message(&mut stream)?;
        let res = match cli_message {
            MessageType::File(path) => handle_file(&path)?,
            MessageType::Image(path) => handle_image(&path)?,
            MessageType::Text => ResponseType::Text(String::from("Server is talking to you :)")),
            MessageType::Quit => {
                let resp = ResponseType::Quit(String::from("Your last desire was disconnection.."));
                send_message(&mut stream, &resp)?;
                stream.shutdown(std::net::Shutdown::Both)?;
                return Ok(());
            }
        };

        println!("before send msg");
        send_message(&mut stream, &res)?;
    }
}

fn handle_file(path: &str) -> Result<ResponseType, Box<dyn Error>> {
    let file_name = get_file_name(path)?;
    println!("IN HF");
    let res = match read_file(path) {
        Ok(contents) => ResponseType::File(file_name, contents),
        Err(e) => {
            println!("Error reading file: {}", e);
            ResponseType::Text(String::from("Error reading file"))
        }
    };

    Ok(res)
}

fn handle_image(path: &str) -> Result<ResponseType, Box<dyn Error>> {
    let file_name = get_file_name(path)?;
    let res = match read_file(path) {
        Ok(contents) => ResponseType::Image(file_name, contents),
        Err(e) => {
            println!("Error reading file: {}", e);
            ResponseType::Text(String::from("Error reading file"))
        }
    };
    Ok(res)
}

fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn get_file_name(path: &str) -> Result<String, Box<dyn Error>> {
    let name = Path::new(path)
        .file_name()
        .and_then(OsStr::to_str)
        .map(String::from)
        .unwrap();
    Ok(name)
}
