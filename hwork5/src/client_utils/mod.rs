use bincode::{deserialize, serialize};

use chrono::Local;
use image::{load_from_memory, ImageFormat};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::{
    error::Error,
    fmt::{self, Display},
    fs::{self, File},
    io,
    net::{Ipv4Addr, TcpStream},
    path::Path,
    str::FromStr,
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

impl MessageType {
    fn valid_options() -> Vec<&'static str> {
        vec![".file", ".image", ".text", ".quit"]
    }
}

#[derive(Debug)]
pub struct ParserErr {
    message: String,
}

impl Display for ParserErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParserErr {}

impl ParserErr {
    fn invalid_option(option: &str) -> ParserErr {
        ParserErr {
            message: format!(
                "Entered invalid option {}. Available options: {}",
                option,
                MessageType::valid_options().join(" / ")
            ),
        }
    }
}

impl FromStr for MessageType {
    type Err = ParserErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.trim().split(' ').collect();
        let option = input[0];

        match option {
            ".file" => Ok(MessageType::File(input[1].to_string())),
            ".image" => Ok(MessageType::Image(input[1].to_string())),
            ".text" => Ok(MessageType::Text),
            ".quit" => Ok(MessageType::Quit),
            _ => Err(ParserErr::invalid_option(option)),
        }
    }
}

pub fn intro() {
    println!("Use input format: <command> <path_to_file>. \nFor .text and .quit <path_to_file> is not needed. \nOptions available:{}", MessageType::valid_options().join(" / "))
}

pub fn parse_input(input: &mut String) -> Result<MessageType, Box<dyn Error>> {
    input.clear();

    println!("Enter: <command> <path_to_file>");

    if let Err(e) = io::stdin().read_line(input) {
        Err(format!("{e}").into())
    } else {
        println!("User input: {input}");
        Ok(input.as_str().parse::<MessageType>()?)
    }
}

fn serialize_message(message: &MessageType) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(serialize(&message)?)
}

fn deserialize_message(data: &[u8]) -> Result<ResponseType, Box<dyn Error>> {
    Ok(deserialize(data)?)
}

pub fn send_message(stream: &mut TcpStream, message: &MessageType) -> Result<(), Box<dyn Error>> {
    let serialized = serialize_message(message)?;

    // Send the length of the serialized message (as 4-byte value).
    let len = serialized.len() as u32;
    stream.write_all(&len.to_be_bytes())?;

    // Send the serialized message.
    stream.write_all(&serialized)?;
    Ok(())
}

pub fn receive_message(stream: &mut TcpStream) -> Result<ResponseType, Box<dyn Error>> {
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

pub fn create_tcp_stream(socket: &[String]) -> Result<TcpStream, Box<dyn Error>> {
    let addr = parse_socket_addr(socket);
    let stream = TcpStream::connect(addr)?;
    Ok(stream)
}

pub fn action(server_resp: &ResponseType) -> Result<(), Box<dyn Error>> {
    match server_resp {
        ResponseType::File(name, content) => {
            println!("Received file with name: {name}");
            save_file(name, content)?;
        }
        ResponseType::Image(name, img) => {
            println!("Received image with name: {name}");
            save_image(img)?;
        }
        ResponseType::Text(content) => {
            println!("Received text: {content}");
        }
        ResponseType::Quit(content) => {
            println!("{content}");
        }
    };

    Ok(())
}

fn save_file(name: &str, content: &[u8]) -> Result<(), Box<dyn Error>> {
    let dir = Path::new("client_db/files");
    fs::create_dir_all(dir)?;

    let path = dir.join(name);

    let mut file = File::create(path)?;
    file.write_all(content)?;

    Ok(())
}

fn save_image(content: &[u8]) -> Result<(), Box<dyn Error>> {
    let dir = Path::new("client_db/images");
    fs::create_dir_all(dir)?;

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let path = dir.join(format!("{}.png", timestamp));

    File::create(&path)?;
    let img = load_from_memory(content)?;

    img.save_with_format(path, ImageFormat::Png)?;

    Ok(())
}
