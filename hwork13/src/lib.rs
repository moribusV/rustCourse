use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use thiserror::Error;

/// Defines the message types client ---> server.
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    File(String),
    Image(String),
    Text(String),
    Quit,
}

/// Defines the response types server ---> client.
#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    File(String, Vec<u8>),
    Image(String, Vec<u8>),
    Text(String),
    Quit(String),
    Error(String),
}

/// Custom error type for message parsing.
#[derive(Error, Debug)]
pub enum SharedLibError {
    #[error("Invalid option: {0}")]
    InvalidOption(String),
    #[error("Missing argument for option: {0}")]
    MissingArgument(String),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
    #[error("Address parsing error: {0}")]
    AddressParsingError(#[from] std::net::AddrParseError),
}

impl FromStr for MessageType {
    type Err = SharedLibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.trim().splitn(2, ' ').collect();
        let option = input[0];

        match option {
            ".file" => input
                .get(1)
                .map(|&path| MessageType::File(path.to_string()))
                .ok_or_else(|| SharedLibError::MissingArgument(option.to_string())),
            ".image" => input
                .get(1)
                .map(|&path| MessageType::Image(path.to_string()))
                .ok_or_else(|| SharedLibError::MissingArgument(option.to_string())),
            ".text" => input
                .get(1)
                .map(|&text| MessageType::Text(text.to_string()))
                .ok_or_else(|| SharedLibError::MissingArgument(option.to_string())),
            ".quit" => Ok(MessageType::Quit),
            _ => Err(SharedLibError::InvalidOption(option.to_string())),
        }
    }
}

/// Serializes a message of type T into a byte vector.
pub fn serialize_message<T: Serialize>(message: &T) -> Result<Vec<u8>, SharedLibError> {
    serialize(message).map_err(SharedLibError::SerializationError)
}

/// Deserializes a byte slice into a message of type T.
pub fn deserialize_message<T: DeserializeOwned>(data: &[u8]) -> Result<T, SharedLibError> {
    deserialize(data).map_err(SharedLibError::SerializationError)
}

/// Sends a serialized message over a TCP stream.
pub fn send_message<T: Serialize>(
    stream: &mut TcpStream,
    message: &T,
) -> Result<(), SharedLibError> {
    let serialized = serialize_message(message)?;
    let len = serialized.len() as u32;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(&serialized)?;
    Ok(())
}

/// Receives a serialized message from a TCP stream.
pub fn receive_message<T: DeserializeOwned>(stream: &mut TcpStream) -> Result<T, SharedLibError> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let exact_len = u32::from_be_bytes(len_buf) as usize;
    let mut message_buf = vec![0u8; exact_len];
    stream.read_exact(&mut message_buf)?;
    deserialize_message(&message_buf)
}

/// Reads and parses user input into a `MessageType`.
pub fn parse_input() -> Result<MessageType, SharedLibError> {
    let mut input = String::new();
    println!("Enter: <command> <path_to_file/text>");
    io::stdin().read_line(&mut input)?;
    input.trim().parse::<MessageType>()
}

/// Parses a socket address from a string.
pub fn parse_socket_addr(val: &str) -> Result<SocketAddr, SharedLibError> {
    SocketAddr::from_str(val).map_err(SharedLibError::AddressParsingError)
}
