use hwork5::client_utils::{
    action, create_tcp_stream, intro, parse_input, receive_message, send_message, ResponseType,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut user_input = String::with_capacity(200);
    let mut stream = match create_tcp_stream(&args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    intro();

    loop {
        let message = match parse_input(&mut user_input) {
            Ok(message) => message,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };
        if let Err(e) = send_message(&mut stream, &message) {
            eprintln!("{e}");
            std::process::exit(1);
        };

        let server_resp = match receive_message(&mut stream) {
            Ok(message) => message,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        };

        if let Err(e) = action(&server_resp) {
            eprintln!("{e}");
            std::process::exit(1);
        };

        if let ResponseType::Quit(_) = server_resp {
            return;
        }
    }
}
