mod days;
mod utils;

use clap::{App, Arg};
use days::*;
use std::process;
use utils::challenge::Challenge;
use utils::input::Input;

fn main() {
    let matches = App::new("Advent of Code 2021")
        .arg(
            Arg::with_name("DAY")
                .help("Sets the day")
                .required(true)
                .index(1)
                .multiple(false),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let day: u32 = matches
        .value_of("DAY")
        .unwrap_or_else(|| {
            eprintln!("Problem parsing DAY: {:#?}", matches);
            process::exit(1);
        })
        .parse::<u32>()
        .unwrap_or_else(|err| {
            eprintln!("Problem converting DAY: {:#?}", err);
            process::exit(1);
        });

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
        1 => {
            let challenge = day1::Day1::new(input.lines);
            challenge.run()
        }
        x => unimplemented!("Invalid day: {}", x),
    }
    .unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });

    println!("The result for day {}:\t\n{}", input.day, result);
}
