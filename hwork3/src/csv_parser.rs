use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use std::fmt;

pub fn parse_csv(input: &String) -> Result<(), Box<dyn Error>> {
    let mut item = Row {
        row: Vec::new(),
        width: len_of_longest_word(input),
    };
    // println!("{}", item.width);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(input.as_bytes());

    for line in rdr.deserialize() {
        item.row = line?;
        println!("{item}");
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Row {
    row: Vec<String>,
    width: usize,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rows = &self.row;
        let mut horizon_sep = String::new();
        let mut counter = 0;

        for v in rows.iter() {
            write!(f, "|{:^width$}|", v, width = self.width)?;
            counter += 1;
        }

        horizon_sep = "-".repeat(self.width * counter + (counter * 2));

        write!(f, "\n{}", horizon_sep)?;

        write!(f, "")
    }
}

fn len_of_longest_word(input: &str) -> usize {
    let re = Regex::new(r"[,\n]").unwrap();
    let max_length = re.split(input).map(|word| word.len()).max().unwrap_or(0);
    max_length
}
