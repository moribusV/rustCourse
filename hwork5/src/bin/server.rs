use hwork5::server_utils::{create_tcp_listener, handle_client};
use std::env;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let listener = match create_tcp_listener(&args) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {e}");
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection fails: {e}");
            }
        }
    }
}
