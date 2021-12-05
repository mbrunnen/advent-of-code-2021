use crate::utils::challenge::Challenge;
use regex::Regex;
use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;

pub struct Day2 {
    data: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pat = Regex::new(r"^(\w+)\s(\d+)$").unwrap();
        let token = pat.captures(s).unwrap();

        let direction = token.get(1).unwrap().as_str();
        let value = token.get(2).unwrap().as_str().parse::<i32>()?;

        let (x, y) = match direction {
            "forward" => (value, 0),
            "down" => (0, value),
            "up" => (0, -value),
            _ => {
                panic!("Unknown direction {}", direction)
            }
        };

        Ok(Self { x, y })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct SubmarineState {
    pos: Point,
    vel: i32,
    aim: i32,
}

impl SubmarineState {
    pub fn new(vel: i32, aim: i32) -> Self {
        Self {
            pos: Point { x: 0, y: 0 },
            vel,
            aim,
        }
    }
}

impl FromStr for SubmarineState {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pat = Regex::new(r"^(\w+)\s(\d+)$").unwrap();
        let token = pat.captures(s).unwrap();

        let direction = token.get(1).unwrap().as_str();
        let value = token.get(2).unwrap().as_str().parse::<i32>()?;

        let (vel, aim) = match direction {
            "forward" => (value, 0),
            "down" => (0, value),
            "up" => (0, -value),
            _ => {
                panic!("Unknown direction {}", direction)
            }
        };

        Ok(Self::new(vel, aim))
    }
}

impl Add for SubmarineState {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            pos: Point {
                x: self.pos.x + other.vel,
                y: self.pos.x + (self.aim * other.vel),
            },
            vel: other.vel,
            aim: self.aim + other.aim,
        }
    }
}

impl Challenge for Day2 {
    fn new(input_file: &str) -> Self {
        Self {
            data: Self::load(input_file).unwrap(),
        }
    }

    fn run(&self, part: u32) -> Result<String, String> {
        match part {
            1 => self.run_part_one(),
            2 => self.run_part_two(),
            x => unimplemented!(
                "Invalid part {} for Day {}",
                x,
                std::any::type_name::<Self>()
            ),
        }
    }
}

impl Day2 {
    fn run_part_one(&self) -> Result<String, String> {
        let end_position: Point = self
            .data
            .iter()
            .map(|l| Point::from_str(l).unwrap())
            .fold(Point { x: 0, y: 0 }, |acc, item| acc + item);

        Ok(format!("{:#?}", end_position.x * end_position.y))
    }

    fn run_part_two(&self) -> Result<String, String> {
        let end_position: SubmarineState = self
            .data
            .iter()
            .map(|l| SubmarineState::from_str(l).unwrap())
            .fold(SubmarineState::new(0, 0), |acc, item| acc + item);

        Ok(format!("{:#?}", end_position.pos.x * end_position.pos.y))
    }
}
