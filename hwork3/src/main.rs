mod utils;

use std::{env, io, io::Read};

use utils::transform_str;

use crate::utils::parse_and_validate_option;

fn main() {
    let args: Vec<String> = env::args().collect();
    let valid_options = vec![
        "lowercase",
        "uppercase",
        "no-spaces",
        "slugify",
        "trim",
        "repeat",
    ];

    println!("Enter text to be transformed:");
    println!("{}", args.len());
    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(_) => {
            println!("User text input: {}", input);
        }
        Err(e) => {
            eprintln!("User input error {}", e);
            panic!("Cannot proceed. Input error.");
        }
    };

    let res_option = match parse_and_validate_option(&args, &valid_options) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Invalid input. Error: {e}. Terminating...");
            panic!("Parsing and validation of program argument not passed.");
        }
    };

    match transform_str(&input, res_option.as_str()) {
        Ok(converted_str) => println!("{converted_str}"),
        Err(e) => eprintln!("{e}"),
    }
}
