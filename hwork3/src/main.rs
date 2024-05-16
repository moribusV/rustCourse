mod csv_parser;
mod utils;

use crate::{csv_parser::parse_csv, utils::parse_and_validate_option};
use std::{env, io, io::Read};
use utils::transform_str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let valid_options = vec![
        "lowercase",
        "uppercase",
        "no-spaces",
        "slugify",
        "trim",
        "repeat",
        "csv",
    ];

    let res_option = match parse_and_validate_option(&args, &valid_options) {
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

    if res_option.as_str() != "csv" {
        match transform_str(&input, res_option.as_str()) {
            Ok(converted_str) => println!("{converted_str}"),
            Err(e) => eprintln!("{e}"),
        }
    } else {
        match parse_csv(&input) {
            Ok(_) => println!("Time to enjoy csv table :)"),
            Err(e) => eprintln!("{e}"),
        }
    }
}
