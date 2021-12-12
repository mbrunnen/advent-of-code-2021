use crate::utils::challenge::Challenge;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
struct DisplayPattern {
    mapping: HashMap<u8, HashSet<char>>,
    input: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl From<&str> for DisplayPattern {
    fn from(line: &str) -> Self {
        let sep: Vec<&str> = line.split('|').collect();

        let input: Vec<HashSet<char>> = sep
            .get(0)
            .unwrap()
            .split(' ')
            .map(|e| HashSet::<char>::from_iter(e.chars()))
            .filter(|e| !e.is_empty())
            .collect();
        let output: Vec<HashSet<char>> = sep
            .get(1)
            .unwrap()
            .split(' ')
            .map(|e| HashSet::<char>::from_iter(e.chars()))
            .filter(|e| !e.is_empty())
            .collect();
        Self {
            mapping: HashMap::new(),
            input,
            output,
        }
    }
}

impl DisplayPattern {
    fn count_out_by_len(&self, len: usize) -> usize {
        self.output
            .iter()
            .fold(0, |acc, item| if item.len() == len { acc + 1 } else { acc })
    }

    fn find_by_len(&self, len: usize) -> HashSet<char> {
        self.input
            .iter()
            .find(|&item| item.len() == len)
            .unwrap()
            .clone()
    }

    fn filter_by_len(&self, len: usize) -> Vec<HashSet<char>> {
        self.input
            .iter()
            .filter(|item| item.len() == len)
            .cloned()
            .collect()
    }

    fn fill_mapping(&mut self) {
        let one = self.find_by_len(2);
        let four = self.find_by_len(4);
        let seven = self.find_by_len(3);
        let eight = self.find_by_len(7);

        // 2 3 5
        let five_segs = self.filter_by_len(5);
        let mut two = HashSet::<char>::new();
        let mut three = HashSet::<char>::new();
        let mut five = HashSet::<char>::new();

        // 0 6 9
        let six_segs = self.filter_by_len(6);
        let mut zero = HashSet::<char>::new();
        let mut six = HashSet::<char>::new();
        let mut nine = HashSet::<char>::new();

        for cand in five_segs {
            if cand.difference(&one).count() == 3 {
                three = cand.clone();
            } else if cand.difference(&four).count() == 2 {
                five = cand.clone();
            } else {
                two = cand.clone();
            }
        }

        for cand in six_segs {
            if cand.difference(&one).count() == 5 {
                six = cand.clone();
            } else if cand.difference(&four).count() == 2 {
                nine = cand.clone();
            } else {
                zero = cand.clone();
            }
        }

        self.mapping.insert(0, zero);
        self.mapping.insert(1, one);
        self.mapping.insert(2, two);
        self.mapping.insert(3, three);
        self.mapping.insert(4, four);
        self.mapping.insert(5, five);
        self.mapping.insert(6, six);
        self.mapping.insert(7, seven);
        self.mapping.insert(8, eight);
        self.mapping.insert(9, nine);
    }

    fn decode_digit(&self, sig: &HashSet<char>) -> usize {
        self.mapping.iter().fold(0, |acc, (num, set)| {
            if sig.symmetric_difference(set).count() == 0 {
                *num as usize
            } else {
                acc
            }
        })
    }

    fn decode(&self) -> usize {
        self.output.iter().enumerate().fold(0, |acc, (i, val)| {
            acc + usize::pow(10, (self.output.len() - i - 1) as u32) * self.decode_digit(val)
        })
    }
}

struct Display {
    data: Vec<DisplayPattern>,
}

impl From<&[String]> for Display {
    fn from(lines: &[String]) -> Self {
        let data: Vec<DisplayPattern> =
            lines.iter().map(|x| DisplayPattern::from(&x[..])).collect();
        Self { data }
    }
}

impl Display {
    fn count_out_by_len(&self) -> usize {
        let mut occurrences = 0;

        for disp in &self.data {
            occurrences += disp.count_out_by_len(2);
            occurrences += disp.count_out_by_len(3);
            occurrences += disp.count_out_by_len(4);
            occurrences += disp.count_out_by_len(7);
        }
        occurrences
    }

    fn deduce(&mut self) -> usize {
        let mut sum = 0;
        for disp in self.data.iter_mut() {
            disp.fill_mapping();
            sum += disp.decode();
        }

        sum
    }
}

pub struct Day8 {
    data: Vec<String>,
}

impl Challenge<String> for Day8 {
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

impl Day8 {
    fn run_part_one(&self) -> Result<String, String> {
        let disp_lines = Display::from(&self.data[..]);
        Ok(format!("{:#?}", disp_lines.count_out_by_len()))
    }

    fn run_part_two(&self) -> Result<String, String> {
        let mut disp_lines = Display::from(&self.data[..]);
        Ok(format!("{:#?}", disp_lines.deduce()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let input: Vec<HashSet<char>> = [
            "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb",
        ]
        .iter()
        .map(|e| HashSet::from_iter(e.chars()))
        .collect();
        let output: Vec<HashSet<char>> = ["fdgacbe", "cefdb", "cefbgd", "gcbe"]
            .iter()
            .map(|e| HashSet::from_iter(e.chars()))
            .collect();
        let expected = DisplayPattern {
            mapping: HashMap::new(),
            input,
            output,
        };
        let disp = DisplayPattern::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        assert_eq!(expected, disp);
    }

    #[test]
    fn test_count_out_by_len() {
        let disp = DisplayPattern::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        let mut occurrences = 0;
        occurrences += disp.count_out_by_len(2);
        occurrences += disp.count_out_by_len(3);
        occurrences += disp.count_out_by_len(4);
        occurrences += disp.count_out_by_len(7);
        assert_eq!(2, occurrences);
    }

    #[test]
    fn test_fill_mapping() {
        let mut disp = DisplayPattern::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );

        disp.fill_mapping();

        let actual = disp.mapping.get(&0).unwrap();
        let expected: HashSet<char> = HashSet::from_iter("cagedb".chars());
        assert!(actual.symmetric_difference(&expected).count() == 0);

        let actual = disp.mapping.get(&1).unwrap();
        let expected: HashSet<char> = HashSet::from_iter("ab".chars());
        assert!(actual.symmetric_difference(&expected).count() == 0);

        let actual = disp.mapping.get(&2).unwrap();
        let expected: HashSet<char> = HashSet::from_iter("gcdfa".chars());
        assert!(actual.symmetric_difference(&expected).count() == 0);

        let actual = disp.mapping.get(&3).unwrap();
        let expected: HashSet<char> = HashSet::from_iter("fbcad".chars());
        assert!(actual.symmetric_difference(&expected).count() == 0);

        let actual = disp.mapping.get(&4).unwrap();
        let expected: HashSet<char> = HashSet::from_iter("eafb".chars());
        assert!(actual.symmetric_difference(&expected).count() == 0);

        let actual = disp.mapping.get(&5).unwrap();
        let expected: HashSet<char> = HashSet::from_iter("cdfbe".chars());
        assert!(actual.symmetric_difference(&expected).count() == 0);

        let actual = disp.mapping.get(&6).unwrap();
        let expected: HashSet<char> = HashSet::from_iter("cdfgeb".chars());
        assert!(actual.symmetric_difference(&expected).count() == 0);

        let actual = disp.mapping.get(&7).unwrap();
        let expected: HashSet<char> = HashSet::from_iter("dab".chars());
        assert!(actual.symmetric_difference(&expected).count() == 0);

        let actual = disp.mapping.get(&8).unwrap();
        let expected: HashSet<char> = HashSet::from_iter("acedgfb".chars());
        assert!(actual.symmetric_difference(&expected).count() == 0);

        let actual = disp.mapping.get(&9).unwrap();
        let expected: HashSet<char> = HashSet::from_iter("cefabd".chars());
        assert!(actual.symmetric_difference(&expected).count() == 0);
    }

    #[test]
    fn test_decode_digit() {
        let mut disp = DisplayPattern::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );

        disp.fill_mapping();
        let input: HashSet<char> = HashSet::from_iter("cdfeb".chars());
        let actual = disp.decode_digit(&input);
        assert_eq!(5, actual);
        let input: HashSet<char> = HashSet::from_iter("fcadb".chars());
        let actual = disp.decode_digit(&input);
        assert_eq!(3, actual);
    }

    #[test]
    fn test_decode() {
        let mut disp = DisplayPattern::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );

        disp.fill_mapping();
        let actual = disp.decode();
        assert_eq!(5353, actual);
    }
}
