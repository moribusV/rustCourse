mod csv_parser;
mod direct_mode;
mod tasks;
mod utils;
use direct_mode::direct_version;
use std::env;
use std::sync::mpsc;
use std::thread;
use tasks::{process_input, receive_input};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let (tx, rx) = mpsc::channel();
        let (tx_repeat, rx_repeat) = mpsc::channel();

        let receive_input_handler = thread::spawn(move || {
            receive_input(tx, rx_repeat);
        });

        let process_input_handler = thread::spawn(move || {
            process_input(rx, tx_repeat);
        });

        receive_input_handler.join().unwrap();
        process_input_handler.join().unwrap();
    } else {
        direct_version(&args);
    }
}
