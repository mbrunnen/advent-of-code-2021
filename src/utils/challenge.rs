use std::fs::File;
use std::io::{prelude::*, BufReader};

pub trait Challenge {
    fn new(input_file: &str) -> Self;
    fn run(&self) -> Result<String, String>;
    fn load(input_file: &str) -> Result<Vec<String>, String> {
        match File::open(input_file) {
            Ok(file) => {
                let buf = BufReader::new(file);
                Ok(buf
                    .lines()
                    .map(|l| l.expect("Could not parse line"))
                    .collect())
            }
            Err(e) => Err(format!("Error while reading the file: {}", e)),
        }
    }
}
