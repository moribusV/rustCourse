use crate::utils::{parse_user_input, transform_str};
use std::{io, io::Read};

pub fn direct_version(args: &[String]) {
    let res_option = match parse_user_input(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Invalid input. Error: {e}. Terminating...");
            std::process::exit(1);
        }
    };

    let mut input = String::new();
    if res_option.as_str() != "csv" {
        println!("Enter text to be transformed:");
        match io::stdin().read_to_string(&mut input) {
            Ok(_) => {
                println!("\nUser text input: \n{}\n\n", input);
            }
            Err(e) => {
                eprintln!("User input error: \n{}", e);
                std::process::exit(1);
            }
        };
    }

    match transform_str(&input, res_option.as_str()) {
        Ok(converted_str) => {
            println!("Converted string:");
            println!("{converted_str}\n");
        }
        Err(e) => eprintln!("{e}"),
    }
}
