use crate::utils::is_valid;
use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fmt::Write;

pub fn parse_csv(input: &String) -> Result<String, Box<dyn Error>> {
    if is_valid(input) {
        let mut item = Row {
            row: Vec::new(),
            width: len_of_longest_word(input),
        };
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(input.as_bytes());
        let mut rows: String = String::new();
        for line in rdr.deserialize() {
            item.row = line?;
            writeln!(rows, "{}", item)?;
        }
        Ok(rows)
    } else {
        Err("Empty string was passed. Cannot parse csv.".into())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Row {
    row: Vec<String>,
    width: usize,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rows = &self.row;
        let mut _horizon_sep = String::new();
        let mut counter = 0;

        for v in rows.iter() {
            write!(f, "|{:^width$}|", v, width = self.width)?;
            counter += 1;
        }

        _horizon_sep = "-".repeat(self.width * counter + (counter * 2));

        write!(f, "\n{}", _horizon_sep)?;

        write!(f, "")
    }
}

fn len_of_longest_word(input: &str) -> usize {
    let re = Regex::new(r"[,\n]").unwrap();
    let max_length = re.split(input).map(|word| word.len()).max().unwrap_or(0);
    max_length
}
