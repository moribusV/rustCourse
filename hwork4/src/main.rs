mod csv_parser;
mod utils;

use crate::utils::parse_user_input;
use std::{env, io, io::Read};
use utils::transform_str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let res_option = match parse_user_input(&args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Invalid input. Error: {e}. Terminating...");
            std::process::exit(1);
        }
    };

    println!("Enter text to be transformed:");
    let mut input = String::new();

    match io::stdin().read_to_string(&mut input) {
        Ok(_) => {
            println!("\nUser text input: \n{}\n\n", input);
        }
        Err(e) => {
            eprintln!("User input error: \n{}", e);
            std::process::exit(1);
        }
    };

    match transform_str(&input, res_option.as_str()) {
        Ok(converted_str) => println!("{converted_str}"),
        Err(e) => eprintln!("{e}"),
    }
}
