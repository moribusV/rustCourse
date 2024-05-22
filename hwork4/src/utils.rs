use crate::csv_parser::parse_csv;
use core::fmt;
use slug::slugify;
use std::{error::Error, io::stdin, str::FromStr};

pub enum Options {
    LowerCase,
    UpperCase,
    NoSpaces,
    Slugify,
    Trim,
    Repeat,
    Csv,
}

impl Options {
    pub fn valid_options() -> &'static [&'static str] {
        &[
            "lowercase",
            "uppercase",
            "no-spaces",
            "slugify",
            "trim",
            "repeat",
            "csv",
        ]
    }
}
impl FromStr for Options {
    type Err = OptionParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lowercase" => Ok(Options::LowerCase),
            "uppercase" => Ok(Options::UpperCase),
            "no-spaces" => Ok(Options::NoSpaces),
            "slugify" => Ok(Options::Slugify),
            "trim" => Ok(Options::Trim),
            "repeat" => Ok(Options::Repeat),
            "csv" => Ok(Options::Csv),
            _ => Err(OptionParseErr::InvalidOption(format!(
                "\nEntered option isn't valid. \nAvailable options: {}",
                Options::valid_options().join(" / ")
            ))),
        }
    }
}

#[derive(Debug)]
pub enum OptionParseErr {
    InvalidOption(String),
}
impl Error for OptionParseErr {}
impl fmt::Display for OptionParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OptionParseErr::InvalidOption(ref input) => write!(f, "{}", input),
        }
    }
}

#[derive(Debug)]
struct InputErr(&'static str);

impl fmt::Display for InputErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for InputErr {}

pub fn is_valid(stdin: &str) -> bool {
    !stdin.trim().is_empty()
}

pub fn parse_user_input(args: &[String]) -> Result<String, Box<dyn Error>> {
    match args.len() {
        2 => Ok(args[1].clone()),
        1 => Err("Program requires 1 command line argument. You have entered 0.".into()),
        _ => Err("More then 1 command line argument entered. Only 1 is required.".into()),
    }
}

pub fn parse_continuous_input(input: &mut String) -> Result<String, Box<dyn Error>> {
    let input_cp = input.clone();
    let mut parts = input_cp.trim().splitn(2, ' ');
    let command = parts.next().ok_or(InputErr("\n<command> is missed."))?;
    if !Options::valid_options().contains(&command) {
        return Err(OptionParseErr::InvalidOption(format!(
            "\nEntered option isn't valid. \nAvailable options: {}",
            Options::valid_options().join(" / ")
        ))
        .into());
    }
    let text = parts.next().ok_or(InputErr("\n<input> is missed."))?;
    input.clear();
    input.push_str(text);
    Ok(command.to_string())
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
        println!("Enter number of repetitions (integer number in range 1 to 15):");
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
    let option = option.parse::<Options>()?;
    match option {
        Options::LowerCase => lowercase(stdin),
        Options::UpperCase => uppercase(stdin),
        Options::NoSpaces => no_spaces(stdin),
        Options::Slugify => slugify_conversion(stdin),
        Options::Trim => trim_conversion(stdin),
        Options::Repeat => repeat(stdin),
        Options::Csv => parse_csv(stdin),
    }
}
