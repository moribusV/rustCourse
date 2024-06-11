// types.rs
use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str::FromStr;

/// Defines the message types that a client can send to the server.
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    File(String),
    Image(String),
    Text(String),
    Quit,
}

/// Defines the response types that the server can send back to the client.
#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    File(String, Vec<u8>),  // Response containing a file with its name and content
    Image(String, Vec<u8>), // Response containing an image with its name and content
    Text(String),           // Response containing a text message
    Quit(String),           // Notification of client disconnection
}

/// Custom error type for message parsing.
#[derive(Debug)]
pub struct ParserErr {
    details: String,
}

impl ParserErr {
    fn invalid_option(option: &str) -> Self {
        Self {
            details: format!("Invalid option: {}", option),
        }
    }
    fn missing_argument(option: &str) -> Self {
        Self {
            details: format!("Missing argument for option: {}", option),
        }
    }
}

impl fmt::Display for ParserErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParserErr {}

impl FromStr for MessageType {
    type Err = ParserErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.trim().splitn(2, ' ').collect();
        let option = input[0];

        match option {
            ".file" => input
                .get(1)
                .map(|&path| MessageType::File(path.to_string()))
                .ok_or_else(|| ParserErr::missing_argument(option)),
            ".image" => input
                .get(1)
                .map(|&path| MessageType::Image(path.to_string()))
                .ok_or_else(|| ParserErr::missing_argument(option)),
            ".text" => input
                .get(1)
                .map(|&text| MessageType::Text(text.to_string()))
                .ok_or_else(|| ParserErr::missing_argument(option)),
            ".quit" => Ok(MessageType::Quit),
            _ => Err(ParserErr::invalid_option(option)),
        }
    }
}

/// Serializes a message into a byte vector.
pub fn serialize_message<T: Serialize>(message: &T) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(serialize(message)?)
}

/// Deserializes a byte slice into a message of type T.
pub fn deserialize_message<T: DeserializeOwned>(data: &[u8]) -> Result<T, Box<dyn Error>> {
    Ok(deserialize(data)?)
}

/// Sends a serialized message over a TCP stream.
pub fn send_message<T: Serialize>(
    stream: &mut TcpStream,
    message: &T,
) -> Result<(), Box<dyn Error>> {
    let serialized = serialize_message(message)?;

    let len = serialized.len() as u32;
    stream.write_all(&len.to_be_bytes())?;

    stream.write_all(&serialized)?;
    Ok(())
}

/// Receives a serialized message from a TCP stream.
pub fn receive_message<T: DeserializeOwned>(stream: &mut TcpStream) -> Result<T, Box<dyn Error>> {
    let mut len_buf = [0u8; 4];

    stream.read_exact(&mut len_buf)?;
    let exac_len = u32::from_be_bytes(len_buf) as usize;

    let mut message_buf = vec![0u8; exac_len];
    stream.read_exact(&mut message_buf)?;

    deserialize_message(&message_buf)
}

/// Reads and parses user input into a `MessageType`.
pub fn parse_inputt() -> Result<MessageType, Box<dyn Error>> {
    let mut input = String::new();
    println!("Enter: <command> <path_to_file/text>");

    if let Err(e) = io::stdin().read_line(&mut input) {
        Err(format!("{e}").into())
    } else {
        println!("User input: {input}");
        input.trim().parse::<MessageType>().map_err(|e| e.into())
    }
}
