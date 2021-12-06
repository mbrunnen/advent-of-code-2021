use crate::utils::challenge::Challenge;
use std::convert::TryInto;

pub struct Day3 {
    data: Vec<String>,
}

impl Challenge for Day3 {
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

impl Day3 {
    fn count_ones(nums: &[u32], bit: usize) -> usize {
        nums.iter()
            .map(|n| n & (1 << bit))
            .filter(|x| x > &0)
            .count()
    }

    fn to_num(s: &str) -> u32 {
        u32::from_str_radix(s, 2).expect("Could not parse line")
    }

    fn convert(d: &[String]) -> Vec<u32> {
        d.iter().map(|s| Self::to_num(s)).collect()
    }

    fn calculate_gamma(nums: &[u32], bitwidth: usize) -> usize {
        Self::to_num(
            &(0..bitwidth)
                .rev()
                .map(|b| {
                    if 2 * Self::count_ones(nums, b) > nums.len() {
                        '1'
                    } else {
                        '0'
                    }
                })
                .collect::<String>(),
        )
        .try_into()
        .unwrap()
    }

    fn calculate_epsilon(gamma: usize, bitwidth: usize) -> usize {
        gamma ^ ((1 << bitwidth) - 1)
    }

    fn run_part_one(&self) -> Result<String, String> {
        let data: Vec<u32> = Self::convert(&self.data);
        let gamma = Self::calculate_gamma(&data, 12);
        let epsilon = Self::calculate_epsilon(gamma, 12);

        Ok(format!("{:#?}", gamma * epsilon))
    }

    fn run_part_two(&self) -> Result<String, String> {
        Ok(format!("{:#?}", 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_num() {
        let input: Vec<String> = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let expected: Vec<u32> = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        for (pos, l) in input.iter().enumerate() {
            assert_eq!(Day3::to_num(l), expected[pos]);
        }
    }

    #[test]
    fn test_count_ones() {
        let input: Vec<u32> = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let expected: Vec<usize> = vec![5, 7, 8, 5, 7];

        for (pos, exp) in expected.iter().enumerate() {
            assert_eq!(Day3::count_ones(&input, pos), *exp);
        }
    }

    #[test]
    fn test_calculate_gamma() {
        let input: Vec<u32> = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let expected = 0b10110;

        assert_eq!(Day3::calculate_gamma(&input, 5), expected);
    }

    #[test]
    fn test_calculate_epsilon() {
        assert_eq!(Day3::calculate_epsilon(22, 5), 9);
    }
}
