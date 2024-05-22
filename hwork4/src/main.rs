mod csv_parser;
mod direct_mode;
mod tasks;
mod utils;

use std::thread;

use direct_mode::direct_version;
use std::env;
use std::sync::mpsc;
use tasks::{task1, task2};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let (tx, rx) = mpsc::channel();
        let (tx_repeat, rx_repeat) = mpsc::channel();

        let handle1 = thread::spawn(move || {
            task1(tx, rx_repeat);
        });

        let handle2 = thread::spawn(move || {
            task2(rx, tx_repeat);
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    } else {
        direct_version(&args);
    }
}
