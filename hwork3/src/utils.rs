use slug::slugify;
use std::{error::Error, io::stdin};

fn parse_arg(valid_opt: &Vec<&str>, user_opt: &str) -> Result<String, Box<dyn Error>> {
    if valid_opt.contains(&user_opt) {
        Ok(user_opt.to_string())
    } else {
        Err(format!(
            "Entered option isn't valid. \n Available options: {:?}",
            valid_opt
        )
        .into())
    }
}

fn is_valid(stdin: &str) -> bool {
    !stdin.is_empty()
}
fn parse_user_input(args: &[String]) -> Result<String, Box<dyn Error>> {
    match args.len() {
        2 => Ok(args[1].clone()),
        1 => Err("Program requires 1 command line argument. You have entered 0.".into()),
        _ => Err("More then 1 command line argument enteres. Only 1 is required.".into()),
    }
}

pub fn parse_and_validate_option(
    args: &[String],
    valid_opt: &Vec<&str>,
) -> Result<String, Box<dyn Error>> {
    let res_arg = parse_user_input(args)?;
    parse_arg(valid_opt, &res_arg)
}

fn lowercase(init_string: &str) -> Result<String, Box<dyn Error>> {
    if is_valid(init_string) {
        Ok(init_string.to_lowercase())
    } else {
        Err("Empty string was passed. Cannot do to_lowercase transformation.".into())
    }
}

fn uppercase(init_string: &str) -> Result<String, Box<dyn Error>> {
    if is_valid(init_string) {
        Ok(init_string.to_uppercase())
    } else {
        Err("Empty string was passed. Cannot do to_uppercase transformation.".into())
    }
}

fn no_spaces(init_string: &str) -> Result<String, Box<dyn Error>> {
    if is_valid(init_string) {
        Ok(init_string.replace(' ', ""))
    } else {
        Err("Empty string was passed. Cannot do no_spaces transformation.".into())
    }
}

fn slugify_conversion(init_string: &String) -> Result<String, Box<dyn Error>> {
    if is_valid(init_string) {
        Ok(slugify(init_string))
    } else {
        Err("Empty string was passed. Cannot do slugify transformation.".into())
    }
}

fn trim_conversion(init_string: &str) -> Result<String, Box<dyn Error>> {
    if is_valid(init_string) {
        Ok(init_string.trim().to_string())
    } else {
        Err("Empty string was passed. Cannot do trim transformation.".into())
    }
}

fn repeat(init_string: &str) -> Result<String, Box<dyn Error>> {
    if is_valid(init_string) {
        println!("Enter number of repatitions (integer number in range 1 to 15):");
        let mut num = String::new();
        stdin().read_line(&mut num)?;

        match num.trim().parse::<usize>() {
            Ok(number) if (1..=15).contains(&number) => Ok(init_string.repeat(number)),
            Ok(number) => {
                Err(format!("Entered number - {number} - is out of permitted range [1..15]").into())
            }
            Err(e) => Err(format!("Parsing error: {e}").into()),
        }
    } else {
        Err("Empty string was passed. Cannot do repeat transformation.".into())
    }
}

pub fn transform_str(stdin: &String, option: &str) -> Result<String, Box<dyn Error>> {
    match option {
        "lowercase" => lowercase(stdin),
        "uppercase" => uppercase(stdin),
        "no-spaces" => no_spaces(stdin),
        "slugify" => slugify_conversion(stdin),
        "trim" => trim_conversion(stdin),
        "repeat" => repeat(stdin),
        _ => Err("Unexpected parameter.".into()),
    }
}
