use crate::utils::challenge::Challenge;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct OceanFloor {
    data: HashMap<(u32, u32), u32>,
}

impl OceanFloor {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add_line(&mut self, start: (u32, u32), end: (u32, u32)) {
        if start.0 == end.0 {
            let x = start.0;
            let yrange = if start.1 > end.1 {
                end.1..=start.1
            } else {
                start.1..=end.1
            };
            for y in yrange {
                *self.data.entry((x, y)).or_insert(0) += 1;
            }
        } else if start.1 == end.1 {
            let y = start.1;
            let xrange = if start.0 > end.0 {
                end.0..=start.0
            } else {
                start.0..=end.0
            };
            for x in xrange {
                *self.data.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    fn count_ovelaps(&self) -> usize {
        self.data.values().filter(|&v| *v > 1).count()
    }
}

impl From<&[String]> for OceanFloor {
    fn from(input: &[String]) -> Self {
        let pat = Regex::new(r"^(\d+),(\d+)\s->\s(\d+),(\d+)$").unwrap();
        let mut floor = OceanFloor::new();

        for l in input {
            let tokens = pat.captures(l).unwrap();
            let x1 = tokens.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let y1 = tokens.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let x2 = tokens.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let y2 = tokens.get(4).unwrap().as_str().parse::<u32>().unwrap();
            floor.add_line((x1, y1), (x2, y2));
        }

        floor
    }
}

pub struct Day5 {
    data: Vec<String>,
}

impl Challenge for Day5 {
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

impl Day5 {
    fn run_part_one(&self) -> Result<String, String> {
        let floor = OceanFloor::from(&self.data[..]);
        Ok(format!("{:#?}", floor.count_ovelaps()))
    }

    fn run_part_two(&self) -> Result<String, String> {
        Ok(format!("{:#?}", 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_line() {
        let expected = OceanFloor {
            data: HashMap::from([
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
        floor.add_line((0, 9), (5, 9));
        floor.add_line((0, 9), (2, 9));
        floor.add_line((9, 4), (3, 4));
        assert_eq!(expected, floor)
    }

    #[test]
    fn test_from() {
        let input: Vec<String> = vec![
            "0,9 -> 5,9".to_string(),
            "8,0 -> 0,8".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "6,4 -> 2,0".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
            "0,0 -> 8,8".to_string(),
            "5,5 -> 8,2".to_string(),
        ];

        let expected = OceanFloor {
            data: HashMap::from([
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
        assert_eq!(expected, floor);
    }

    #[test]
    fn test_count_overlaps() {
        let input: Vec<String> = vec![
            "0,9 -> 5,9".to_string(),
            "8,0 -> 0,8".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "6,4 -> 2,0".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
            "0,0 -> 8,8".to_string(),
            "5,5 -> 8,2".to_string(),
        ];

        let floor = OceanFloor::from(&input[..]);
        assert_eq!(5, floor.count_ovelaps());
    }
}
