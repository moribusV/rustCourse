use slug::slugify;
use std::env;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();
    let available_options = vec![
        "lowercase",
        "uppercase",
        "no-spaces",
        "slugify",
        "trim",
        "repeat",
    ];

    if args.len() != 2 {
        println!("Wrong CLI arguments input. Should be 1 argument. \nYou can choose from: {:?} \nTerminating...", available_options);
        std::process::exit(1);
    }

    println!("Enter your text line:");
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Failed to read line.");

    println!("String before conversion: {}", line);
    let option = &args[1];

    let converted_str = match option.as_str() {
        "lowercase" => line.to_lowercase(),
        "uppercase" => line.to_uppercase(),
        "no-spaces" => line.replace(' ', ""),
        "slugify" => slugify(line),
        "trim" => String::from(line.trim()),
        "repeat" => line.repeat(3),
        _ => {
            eprintln!(
                "Error: Entered option for the conversion doesn't match any existing case.\
            \nAvailable options: {:?}",
                available_options
            );
            std::process::exit(1);
        }
    };

    println!("String after conversion: \n{converted_str}");
}
