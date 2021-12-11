use crate::utils::challenge::Challenge;

pub struct Day3 {
    data: Vec<String>,
}

impl Challenge <String>for Day3 {
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

    fn get_most_common_bit(nums: &[u32], bit: usize) -> bool {
        2 * Self::count_ones(nums, bit) >= nums.len()
    }

    fn calculate_gamma(nums: &[u32], bitwidth: usize) -> usize {
        (0..bitwidth).rev().fold(0, |acc, b| {
            if Self::get_most_common_bit(nums, b) {
                acc | (1 << b)
            } else {
                acc
            }
        })
    }

    fn calculate_epsilon(gamma: usize, bitwidth: usize) -> usize {
        gamma ^ ((1 << bitwidth) - 1)
    }

    fn filter_by_bit(nums: Vec<u32>, bit: usize, filter: bool) -> Vec<u32> {
        nums.iter()
            .cloned()
            .filter(|n| (*n & (1 << bit)) == ((filter as u32) << bit))
            .collect()
    }

    fn calculate_oxygen(nums: Vec<u32>, bitwidth: usize) -> Vec<u32> {
        if nums.len() == 1 || bitwidth == 0 {
            return nums;
        }

        let bit = bitwidth - 1;
        let mcb: bool = Self::get_most_common_bit(&nums, bit);
        let new_nums = Self::filter_by_bit(nums, bit, mcb);

        Self::calculate_oxygen(new_nums, bitwidth - 1)
    }

    fn calculate_co2(nums: Vec<u32>, bitwidth: usize) -> Vec<u32> {
        if nums.len() == 1 || bitwidth == 0 {
            return nums;
        }
        let bit = bitwidth - 1;
        let lcb: bool = !Self::get_most_common_bit(&nums, bit);
        let new_nums = Self::filter_by_bit(nums, bit, lcb);

        Self::calculate_co2(new_nums, bitwidth - 1)
    }

    fn run_part_one(&self) -> Result<String, String> {
        let data: Vec<u32> = Self::convert(&self.data);
        let gamma = Self::calculate_gamma(&data, 12);
        let epsilon = Self::calculate_epsilon(gamma, 12);

        Ok(format!("{:#?}", gamma * epsilon))
    }

    fn run_part_two(&self) -> Result<String, String> {
        let data: Vec<u32> = Self::convert(&self.data);
        let oxygen = Self::calculate_oxygen(data.clone(), 12)[0];
        let co2 = Self::calculate_co2(data.clone(), 12)[0];

        Ok(format!("{:#?}", oxygen * co2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<u32> {
        vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ]
    }

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

        let expected = get_input();

        for (pos, l) in input.iter().enumerate() {
            assert_eq!(Day3::to_num(l), expected[pos]);
        }
    }

    #[test]
    fn test_count_ones() {
        let input = get_input();

        let expected: Vec<usize> = vec![5, 7, 8, 5, 7];

        for (pos, exp) in expected.iter().enumerate() {
            assert_eq!(Day3::count_ones(&input, pos), *exp);
        }
    }

    #[test]
    fn test_calculate_gamma() {
        let input = get_input();

        let expected = 0b10110;

        assert_eq!(Day3::calculate_gamma(&input, 5), expected);
    }

    #[test]
    fn test_calculate_epsilon() {
        assert_eq!(Day3::calculate_epsilon(22, 5), 9);
    }

    #[test]
    fn test_get_most_common_bit() {
        assert!(!Day3::get_most_common_bit(&[0, 0], 0));
        assert!(Day3::get_most_common_bit(&[0, 1], 0));
        assert!(Day3::get_most_common_bit(&[1, 1], 0));
    }

    #[test]
    fn test_calculate_oxygen() {
        let input = get_input();

        let expected = 23;
        let oxygen = Day3::calculate_oxygen(input, 5)[0];

        assert_eq!(oxygen, expected);
    }

    #[test]
    fn test_calculate_co2() {
        let input = get_input();

        let expected = 10;
        let oxygen = Day3::calculate_co2(input, 5)[0];

        assert_eq!(oxygen, expected);
    }
}
