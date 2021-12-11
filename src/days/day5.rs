use crate::utils::challenge::Challenge;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct OceanFloor {
    vents: HashMap<(i32, i32), u32>,
}

impl OceanFloor {
    fn new() -> Self {
        Self {
            vents: HashMap::new(),
        }
    }

    fn add_vent(&mut self, start: (i32, i32), end: (i32, i32)) {
        let steps = i32::max(i32::abs(start.0 - end.0), i32::abs(start.1 - end.1));
        let dx = i32::signum(end.0 - start.0);
        let dy = i32::signum(end.1 - start.1);
        for i in 0..=steps {
            *self
                .vents
                .entry((start.0 + i * dx, start.1 + i * dy))
                .or_insert(0) += 1;
        }
    }

    fn count_overlaps(&self) -> usize {
        self.vents.values().filter(|&v| *v > 1).count()
    }
}

impl From<&[String]> for OceanFloor {
    fn from(input: &[String]) -> Self {
        let pat = Regex::new(r"^(\d+),(\d+)\s->\s(\d+),(\d+)$").unwrap();
        let mut floor = OceanFloor::new();

        for l in input {
            let tokens = pat.captures(l).unwrap();
            let x1 = tokens.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y1 = tokens.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let x2 = tokens.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let y2 = tokens.get(4).unwrap().as_str().parse::<i32>().unwrap();
            floor.add_vent((x1, y1), (x2, y2));
        }

        floor
    }
}

pub struct Day5 {
    data: Vec<String>,
}

impl Challenge <String>for Day5 {
    fn new(input_file: &str) -> Self {
        Self {
            data: Self::load(input_file).unwrap(),
        }
    }

    fn run(&self, part: u32) -> Result<String, String> {
        match part {
            2 => self.run_part_two(),
            x => unimplemented!(
                "Invalid part {} for Day {}",
                x,
                std::any::type_name::<Self>()
            ),
        }
    }
}

impl Day5 {
    fn run_part_two(&self) -> Result<String, String> {
        let floor = OceanFloor::from(&self.data[..]);
        Ok(format!("{:#?}", floor.count_overlaps()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_line() {
        let expected = OceanFloor {
            vents: HashMap::from([
                ((0, 9), 2),
                ((1, 9), 2),
                ((2, 9), 2),
                ((3, 9), 1),
                ((4, 9), 1),
                ((5, 9), 1),
                ((9, 4), 1),
                ((8, 4), 1),
                ((7, 4), 1),
                ((6, 4), 1),
                ((5, 4), 1),
                ((4, 4), 1),
                ((3, 4), 1),
            ]),
        };

        let mut floor = OceanFloor::new();
        floor.add_vent((0, 9), (5, 9));
        floor.add_vent((0, 9), (2, 9));
        floor.add_vent((9, 4), (3, 4));
        assert_eq!(expected, floor)
    }

    #[test]
    fn test_from() {
        let input: Vec<String> = vec![
            "0,9 -> 5,9".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
        ];

        let expected = OceanFloor {
            vents: HashMap::from([
                ((1, 4), 1),
                ((1, 9), 2),
                ((5, 9), 1),
                ((5, 4), 1),
                ((3, 9), 1),
                ((2, 1), 1),
                ((7, 3), 1),
                ((7, 0), 1),
                ((7, 2), 1),
                ((2, 4), 1),
                ((7, 4), 2),
                ((6, 4), 1),
                ((4, 9), 1),
                ((2, 9), 2),
                ((2, 2), 1),
                ((9, 4), 1),
                ((4, 4), 1),
                ((8, 4), 1),
                ((7, 1), 1),
                ((0, 9), 2),
                ((3, 4), 2),
            ]),
        };

        let floor = OceanFloor::from(&input[..]);
        assert_eq!(expected.vents, floor.vents);
    }

    #[test]
    fn test_count_straight_overlaps() {
        let input: Vec<String> = vec![
            String::from("0,9 -> 5,9"),
            String::from("8,0 -> 0,8"),
            String::from("9,4 -> 3,4"),
            String::from("2,2 -> 2,1"),
            String::from("7,0 -> 7,4"),
            String::from("6,4 -> 2,0"),
            String::from("0,9 -> 2,9"),
            String::from("3,4 -> 1,4"),
            String::from("0,0 -> 8,8"),
            String::from("5,5 -> 8,2"),
        ];

        let floor = OceanFloor::from(&input[..]);
        assert_eq!(12, floor.count_overlaps());
    }

    #[test]
    fn test_add_diagonal() {
        let expected = OceanFloor {
            vents: HashMap::from([
                ((0, 0), 1),
                ((0, 8), 1),
                ((1, 1), 1),
                ((1, 7), 1),
                ((2, 0), 1),
                ((2, 2), 1),
                ((2, 6), 1),
                ((3, 1), 1),
                ((3, 3), 1),
                ((3, 5), 1),
                ((4, 2), 1),
                ((4, 4), 2),
                ((5, 3), 2),
                ((5, 5), 1),
                ((6, 2), 1),
                ((6, 4), 1),
                ((6, 6), 1),
                ((7, 1), 1),
                ((7, 7), 1),
                ((8, 0), 1),
                ((8, 8), 1),
            ]),
        };

        let mut floor = OceanFloor::new();
        floor.add_vent((8, 0), (0, 8));
        floor.add_vent((6, 4), (2, 0));
        floor.add_vent((0, 0), (8, 8));
        assert_eq!(expected, floor)
    }
}
