use slug::slugify;
use std::env;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Haven't received CLI arguments. Terminating...");
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
        _ => String::from("Entered option for the conversion doesn't match any existing case."),
    };

    println!("String after conversion: {}", converted_str);
}
