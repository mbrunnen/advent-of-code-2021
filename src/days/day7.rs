use crate::utils::challenge::Challenge;
use std::collections::HashMap;

pub struct Day7 {
    data: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct CrabMap {
    positions: HashMap<isize, usize>,
}

impl From<&str> for CrabMap {
    fn from(input: &str) -> Self {
        let crabs: Vec<isize> = input
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect();

        let mut positions = HashMap::<isize, usize>::new();
        for pos in crabs {
            *positions.entry(pos).or_insert(0) += 1;
        }

        Self { positions }
    }
}

impl CrabMap {
    fn align_to(&self, pos: isize) -> isize {
        self.positions
            .iter()
            .map(|(p, c)| isize::abs((pos - p) * *c as isize))
            .sum::<isize>()
    }

    fn max_position(&self) -> isize {
        *self.positions.keys().max().unwrap()
    }

    fn min_position(&self) -> isize {
        *self.positions.keys().min().unwrap()
    }

    fn optimise(&self) -> isize {
        (self.min_position()..=self.max_position())
            .map(|pos| self.align_to(pos))
            .min()
            .unwrap()
    }
}

impl Challenge<String> for Day7 {
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

impl Day7 {
    fn run_part_one(&self) -> Result<String, String> {
        let crabs = CrabMap::from(&self.data[0][..]);
        Ok(format!("{:#?}", crabs.optimise()))
    }

    fn run_part_two(&self) -> Result<String, String> {
        Ok(format!("{:#?}", 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let expected = CrabMap {
            positions: HashMap::from([(0, 1), (1, 2), (2, 3), (4, 1), (7, 1), (14, 1), (16, 1)]),
        };
        let crabs = CrabMap::from("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(expected, crabs);
    }

    #[test]
    fn test_align_to() {
        let crabs = CrabMap::from("16,1,2,0,4,2,7,1,2,14");
        let fuel = crabs.align_to(1);
        assert_eq!(41, fuel);
        let fuel = crabs.align_to(2);
        assert_eq!(37, fuel);
        let fuel = crabs.align_to(3);
        assert_eq!(39, fuel);
    }

    #[test]
    fn test_max_position() {
        let crabs = CrabMap::from("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(16, crabs.max_position());
    }

    #[test]
    fn test_optimise() {
        let crabs = CrabMap::from("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(37, crabs.optimise());
    }
}
