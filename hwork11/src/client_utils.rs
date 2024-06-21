use chrono::Local;
use hwork11::{receive_message, ResponseType};
use image::{load_from_memory, ImageFormat};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use tracing::info;

pub fn handle_server(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        let response = receive_message::<ResponseType>(&mut stream)?;

        match response {
            ResponseType::File(name, content) => {
                info!("Received file with name: {name}");
                save_file(&name, &content)?;
            }
            ResponseType::Image(name, img) => {
                info!("Received image with name: {name}");
                save_image(&img)?;
            }
            ResponseType::Text(msg) => {
                info!("Server: {}", msg);
            }
            ResponseType::Quit(addr) => {
                info!("{} has disconnected", addr);
                break;
            }
        }
    }

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

    let img = load_from_memory(content)?;
    img.save_with_format(path, ImageFormat::Png)?;

    Ok(())
}
