use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub struct Input {
    pub day: usize,
    pub lines: Vec<String>,
}

impl Input {
    pub fn load(args: &[String]) -> Result<Self, String> {
        if args.len() != 3 {
            return Err(format!("Usage: {} DAY INPUT!", &args[0]));
        }

        let day = args[1].parse::<usize>().unwrap();
        let filename = &args[2];

        match File::open(filename) {
            Ok(file) => {
                let buf = BufReader::new(file);
                Ok(Input {
                    day,
                    // XXX: Here is no advantage. The users can call self.input.lines() instead of self.lines
                    lines: buf
                        .lines()
                        .map(|l| l.expect("Could not parse line"))
                        .collect(),
                })
            }
            Err(e) => Err(format!("Error while reading the file: {}", e)),
        }
    }
}
