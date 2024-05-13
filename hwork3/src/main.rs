mod utils;

use std::{env, io};

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
    let stdin = match io::read_to_string(io::stdin()) {
        Ok(user_in) => {
            println!("User text input: {}", user_in);
            user_in
        }
        Err(e) => {
            eprintln!("User input error {}", e);
            "Default string! Lets give it more chance.".to_string()
        }
    };

    let res_option = match parse_and_validate_option(&args, &valid_options) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Invalid input. Error: {e}. Terminating...");
            panic!("Parsing and validation of program argument not passed.");
        }
    };

    match transform_str(&stdin, res_option.as_str()) {
        Ok(converted_str) => println!("{converted_str}"),
        Err(e) => eprintln!("{e}"),
    }
}
