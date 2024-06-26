use anyhow::{Context, Result};
use hwork15::{receive_message, send_message, MessageType, ResponseType};
use std::{net::SocketAddr, sync::Arc};
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::db::Database;

pub async fn handle_client(
    mut stream: OwnedReadHalf,
    stream_w: &Arc<Mutex<OwnedWriteHalf>>,
    addr: SocketAddr,
    sender: Sender<(ResponseType, SocketAddr)>,
    database: Arc<Database>,
) -> Result<()> {
    let username = handle_authentication_or_registration(
        &mut stream,
        stream_w.clone(),
        addr,
        database.clone(),
    )
    .await?;
    info!("User {username} authenticated.");
    loop {
        let cli_message = match receive_message::<MessageType, OwnedReadHalf>(&mut stream).await {
            Ok(msg) => msg,
            Err(e) => {
                error!("Error receiving message from {}: {:?}", addr, e);
                break;
            }
        };

        let res = match cli_message {
            MessageType::File(path) => handle_file(&path).await.unwrap_or_else(|e| {
                ResponseType::Error(format!("Error handling file{}: {}", path, e))
            }),
            MessageType::Image(path) => handle_image(&path).await.unwrap_or_else(|e| {
                ResponseType::Error(format!("Error handling image {}: {}", path, e))
            }),
            MessageType::Text(text) => {
                if let Err(e) = database.save_message_by_username(&username, &text).await {
                    error!("Failed to save message to database: {:?}", e);
                }
                ResponseType::Text(format!("{}: {}", username, text))
            }
            MessageType::Quit => {
                info!("Client {} has disconnected.", addr);
                break;
            }
        };

        if sender.send((res, addr)).is_err() {
            break;
        }
    }

    Ok(())
}

async fn handle_file(path: &str) -> Result<ResponseType> {
    let file_name = get_file_name(path).context("Failed to get file name")?;
    let contents = read_file(path).await.context("Failed to read file")?;
    Ok(ResponseType::File(file_name, contents))
}

async fn handle_image(path: &str) -> Result<ResponseType> {
    let file_name = get_file_name(path).context("Failed to get image name")?;
    let contents = read_file(path).await.context("Failed to read image")?;
    Ok(ResponseType::Image(file_name, contents))
}

async fn read_file(path: &str) -> Result<Vec<u8>> {
    let mut file = fs::File::open(path).await.context("Failed to open file")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .await
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

async fn handle_authentication_or_registration(
    stream: &mut OwnedReadHalf,
    stream_w: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
    addr: std::net::SocketAddr,
    database: Arc<Database>,
) -> Result<String> {
    loop {
        info!("Server is ready to authenticate you.");
        let auth_message = match receive_message::<String, OwnedReadHalf>(stream).await {
            Ok(msg) => msg,
            Err(e) => {
                error!("Error receiving auth message from {}: {:?}", addr, e);
                return Err(e.into());
            }
        };

        let parts: Vec<&str> = auth_message.split_whitespace().collect();
        if parts.len() != 3 || (parts[0] != "AUTH" && parts[0] != "REGISTER") {
            return Err(anyhow::anyhow!("Invalid auth/registration message format"));
        }

        let action = parts[0];
        let username = parts[1];
        let password = parts[2];

        if action == "REGISTER" {
            match database.create_user(username, password).await {
                Ok(_) => {
                    let mut stream = stream_w.lock().await;
                    send_message(
                        &mut *stream,
                        &ResponseType::Text("Registration successful".to_string()),
                    )
                    .await?;
                    drop(stream);
                    return Ok(username.to_string());
                }
                Err(e) => {
                    error!("Registration failed for {}: {:?}", addr, e);
                    let mut stream = stream_w.lock().await;
                    send_message(
                        &mut *stream,
                        &ResponseType::Error("Registration failed".to_string()),
                    )
                    .await?;
                    drop(stream);
                }
            }
        } else if action == "AUTH" {
            match database.authenticate_user(username, password).await {
                Ok(_) => {
                    let mut stream = stream_w.lock().await;
                    send_message(&mut *stream, &ResponseType::Text("AUTH OK".to_string())).await?;
                    drop(stream);
                    return Ok(username.to_string());
                }
                Err(e) => {
                    error!("Authentication failed for {}: {:?}", addr, e);
                    let failure_message = ResponseType::Error(format!("AUTH FAILED: {:?}", e));

                    let mut stream = stream_w.lock().await;
                    send_message(&mut *stream, &failure_message).await?;
                    drop(stream);
                }
            }
        }
    }
}
