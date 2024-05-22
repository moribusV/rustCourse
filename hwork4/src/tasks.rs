use crate::utils::{parse_continuous_input, transform_str};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self};
use std::sync::Arc;
use std::{io, io::Read};

pub fn task1(tx: mpsc::Sender<Vec<String>>, rx: mpsc::Receiver<bool>, stop_flag: Arc<AtomicBool>) {
    let mut input = String::new(); // initially string containes command and input separated by whitespace
    let mut msg = Vec::new();
    while !stop_flag.load(Ordering::SeqCst) {
        msg.clear();
        println!("Enter <command> <input>:");

        let command = io::stdin()
            .read_to_string(&mut input)
            .map_err(|e| {
                eprintln!("{e}");
                std::process::exit(1);
            })
            .and_then(|_| parse_continuous_input(&mut input))
            .map_err(|e| {
                eprintln!("{e}");
                std::process::exit(1);
            })
            .unwrap();

        msg.push(command.clone());
        msg.push(input.clone());

        if let Err(e) = tx.send(msg.clone()) {
            eprintln!("{e}");
            std::process::exit(1);
        }
        while let Ok(res) = rx.recv() {
            if res {
                break;
            }
        }
    }
}

pub fn task2(rx: mpsc::Receiver<Vec<String>>, tx: mpsc::Sender<bool>, stop_flag: Arc<AtomicBool>) {
    while !stop_flag.load(Ordering::SeqCst) {
        let result = match rx.recv() {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        };

        let command = result[0].clone();
        let text = result[1].clone();

        match transform_str(&text, command.as_str()) {
            Ok(converted_str) => {
                println!("Converted string:");
                println!("{converted_str}\n");

                if let Err(e) = tx.send(true) {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}
