mod csv_parser;
mod direct_mode;
mod tasks;
mod utils;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use direct_mode::direct_version;
use std::env;
use std::sync::mpsc;
use tasks::{task1, task2};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let (tx, rx) = mpsc::channel();
        let (tx_repeat, rx_repeat) = mpsc::channel();

        let stop_flag1 = stop_flag.clone();
        let handle1 = thread::spawn(move || {
            task1(tx, rx_repeat, stop_flag1);
        });

        let stop_flag2 = stop_flag.clone();
        let handle2 = thread::spawn(move || {
            task2(rx, tx_repeat, stop_flag2);
        });

        {
            let stop_flag = stop_flag.clone();
            ctrlc::set_handler(move || {
                stop_flag.store(true, Ordering::SeqCst);
            })
            .expect("Err setting handler");
        }

        while !stop_flag.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(1));
        }

        handle1.join().unwrap();
        handle2.join().unwrap();
    } else {
        direct_version(&args);
    }
}
