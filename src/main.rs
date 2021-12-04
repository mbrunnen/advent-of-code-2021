mod utils;
mod days;

use clap::{App, Arg};
use std::process;
use utils::input::Input;
use utils::challenge::Challenge;
use days::*;

fn main() {
    let matches = App::new("Advent of Code 2021")
        .arg(
            Arg::with_name("DAY")
                .help("Sets the day")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let day = matches.value_of("DAY");
    let input_files: Vec<_> = matches
        .values_of("INPUT")
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    let input = Input::load(&input_files[..]).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let result = match day {
        Some("1") => {
            let challenge = day1::Day1::new(input.lines);
            challenge.run()
        }
        Some(x) => unimplemented!("Invalid day: {}", x),
        None => {
            eprintln!("Problem parsing arguments: {:#?}", matches);
            process::exit(1);
        }
    }
    .unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });

    println!("The result for day {}:\t\n{}", input.day, result);
}
