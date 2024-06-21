use anyhow::{Context, Result};
use chrono::Local;
use hwork13::{receive_message, ResponseType};
use image::{load_from_memory, ImageFormat};
use std::fs::{self, File};
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use tracing::{error, info};

pub fn handle_server(mut stream: TcpStream) -> Result<()> {
    loop {
        let response =
            receive_message::<ResponseType>(&mut stream).context("Failed to receive message")?;

        match response {
            ResponseType::File(name, content) => {
                info!("Received file with name: {name}");
                save_file(&name, &content).context("Failed to save file")?;
            }
            ResponseType::Image(name, img) => {
                info!("Received image with name: {name}");
                save_image(&img).context("Failed to save image")?;
            }
            ResponseType::Text(msg) => {
                info!("Server: {}", msg);
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

fn save_file(name: &str, content: &[u8]) -> Result<()> {
    let dir = Path::new("client_db/files");
    fs::create_dir_all(dir).context("Failed to create directory.")?;

    let path = dir.join(name);

    let mut file = File::create(path).context("Failed to create file.")?;
    file.write_all(content)?;

    Ok(())
}

fn save_image(content: &[u8]) -> Result<()> {
    let dir = Path::new("client_db/images");
    fs::create_dir_all(dir).context("Failed to create directory.")?;

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let path = dir.join(format!("{}.png", timestamp));

    let img = load_from_memory(content).context("Failed to load context from memory")?;
    img.save_with_format(path, ImageFormat::Png)
        .context("Failed to asve image with png format")?;

    Ok(())
}
