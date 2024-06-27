use anyhow::{Context, Result};
use chrono::Local;
use hwork15::{receive_message, send_message, ResponseType};
use image::{load_from_memory, ImageFormat};
use std::path::Path;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use tokio::task;
use tokio::{fs, io::BufReader};
use tracing::{error, info};

pub async fn handle_server(
    stream_r: OwnedReadHalf,
    stream_w: &Arc<Mutex<OwnedWriteHalf>>,
    lines: &mut tokio::io::Lines<BufReader<tokio::io::Stdin>>,
) -> Result<()> {
    let stream_reader_sync = Arc::new(Mutex::new(stream_r));

    handle_authentication_or_registration(&stream_reader_sync.clone(), stream_w, lines).await?;
    info!("Authentication successful. I was waiting on you.. Neo.");

    loop {
        let mut stream = stream_reader_sync.lock().await;
        let response = receive_message::<ResponseType, OwnedReadHalf>(&mut stream)
            .await
            .context("Failed to receive message")?;
        drop(stream);
        match response {
            ResponseType::File(name, content) => {
                info!("Received file with name: {name}");
                save_file(&name, &content)
                    .await
                    .context("Failed to save file")?;
            }
            ResponseType::Image(name, img) => {
                info!("Received image with name: {name}");
                save_image(&img).await.context("Failed to save image")?;
            }
            ResponseType::Text(msg) => {
                info!("Chat: {}", msg);
            }
            ResponseType::Quit(addr) => {
                info!("{} has disconnected", addr);
                break;
            }
            ResponseType::Error(msg) => {
                error!("Server: {}", msg);
            }
        }
    }

    Ok(())
}

pub async fn save_file(name: &str, content: &[u8]) -> Result<()> {
    let dir = Path::new("client_db/files");
    fs::create_dir_all(dir)
        .await
        .context("Failed to create directory.")?;

    let path = dir.join(name);

    let mut file = fs::File::create(path)
        .await
        .context("Failed to create file.")?;
    file.write_all(content)
        .await
        .context("Failed to write to file.")?;

    Ok(())
}

pub async fn save_image(content: &[u8]) -> Result<()> {
    let dir = Path::new("client_db/images");
    fs::create_dir_all(dir)
        .await
        .context("Failed to create directory.")?;

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let path = dir.join(format!("{}.png", timestamp));

    let content_cp = content.to_vec();
    // Spawn blocking to handle the image processing
    let img = task::spawn_blocking(move || {
        load_from_memory(&content_cp).context("Failed to load content from memory")
    })
    .await??;

    // Save image on a blocking thread
    task::spawn_blocking(move || {
        img.save_with_format(path, ImageFormat::Png)
            .context("Failed to save image with PNG format")
    })
    .await??;

    Ok(())
}

pub async fn handle_authentication_or_registration(
    stream_r: &Arc<Mutex<OwnedReadHalf>>,
    stream_w: &Arc<Mutex<OwnedWriteHalf>>,
    lines: &mut tokio::io::Lines<BufReader<tokio::io::Stdin>>,
) -> Result<()> {
    loop {
        println!("Enter command (REGISTER or AUTH) followed by username and password:");
        let line = lines
            .next_line()
            .await
            .context("Failed to read input line")?
            .context("No input received")?;

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 && (parts[0] == "REGISTER" || parts[0] == "AUTH") {
            let command = parts[0];
            let username = parts[1];
            let password = parts[2];

            let auth_message = format!("{} {} {}", command, username, password);
            println!("{auth_message}");
            let mut writer = stream_w.lock().await;
            send_message(&mut *writer, &auth_message).await?;
            let mut stream = stream_r.lock().await;
            let response: ResponseType = receive_message(&mut *stream).await?;
            match response {
                ResponseType::Text(msg)
                    if msg.contains("AUTH OK") || msg.contains("Registration successful") =>
                {
                    info!("Server response: {}", msg);
                    info!("You are now authenticated.");
                    return Ok(());
                }
                ResponseType::Error(err) => {
                    error!("Authentication or Registration failed: {}", err);
                }
                _ => {
                    error!("Unexpected server response.");
                }
            }
        } else {
            error!("Invalid command. Use REGISTER or AUTH followed by username and password.");
        }
    }
}
