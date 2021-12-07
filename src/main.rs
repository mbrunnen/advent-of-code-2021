mod days;
mod utils;

use clap::{App, Arg};
use days::*;
use std::process;
use utils::challenge::Challenge;

fn parse_arg<T>(arg: Option<&str>) -> Result<T, String>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let arg_str = match arg {
        Some(x) => x,
        None => return Err(format!("Problem parsing argument: {:?}", arg)),
    };

    match arg_str.parse::<T>() {
        Ok(x) => Ok(x),
        Err(err) => Err(format!("Problem parsing argument: {:?}", err)),
    }
}

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
            Arg::with_name("part")
                .short("-p")
                .help("Set the part")
                .default_value("1")
                .multiple(false),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input")
                .required(true)
                .index(2)
                .multiple(false),
        )
        .get_matches();

    let day: u32 = parse_arg(matches.value_of("DAY")).unwrap_or_else(|err| {
        eprintln!("Problem converting DAY: {:?}", err);
        process::exit(1);
    });

    let part: u32 = parse_arg(matches.value_of("part")).unwrap_or_else(|err| {
        eprintln!("Problem converting part: {:?}", err);
        process::exit(1);
    });

    let input_file: String = matches
        .values_of("INPUT")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    let result = match day {
        1 => day1::Day1::new(&input_file).run(part),
        2 => day2::Day2::new(&input_file).run(part),
        3 => day3::Day3::new(&input_file).run(part),
        4 => day4::Day4::new(&input_file).run(part),
        x => unimplemented!("Invalid day: {}", x),
    }
    .unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });

    println!("The result for day {}, part {}: {}", day, part, result);
}
