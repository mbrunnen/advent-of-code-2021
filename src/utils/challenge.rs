use std::fs::File;
use std::io::{prelude::*, BufReader};

pub trait Challenge<T> {
    fn new(input_file: &str) -> Self;
    fn run(&self, part: u32) -> Result<String, String>;
    fn load(input_file: &str) -> Result<Vec<T>, String>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        match File::open(input_file) {
            Ok(file) => {
                let buf = BufReader::new(file);
                Ok(buf
                    .lines()
                    .map(|l| {
                        l.expect("Could not parse line")
                            .parse::<T>()
                            .expect("Could not parse line")
                    })
                    .collect())
            }
            Err(e) => Err(format!("Error while reading the file: {}", e)),
        }
    }
}
