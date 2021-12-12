use crate::utils::challenge::Challenge;

#[derive(Debug, PartialEq)]
struct DisplayPattern {
    input: Vec<String>,
    output: Vec<String>,
}

impl From<&str> for DisplayPattern {
    fn from(line: &str) -> Self {
        let sep: Vec<&str> = line.split('|').collect();
        let input: Vec<String> = sep
            .get(0)
            .unwrap()
            .split(' ')
            .map(|e| e.to_string())
            .filter(|e| !e.is_empty())
            .collect();
        let output: Vec<String> = sep
            .get(1)
            .unwrap()
            .split(' ')
            .map(|e| e.to_string())
            .filter(|e| !e.is_empty())
            .collect();
        Self { input, output }
    }
}

impl DisplayPattern {
    fn count_out_by_len(&self, len: usize) -> usize {
        self.output
            .iter()
            .fold(0, |acc, item| if item.len() == len { acc + 1 } else { acc })
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
        Ok(format!("{:#?}", 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let input: Vec<String> = [
            "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb",
        ]
        .iter()
        .map(|e| e.to_string())
        .collect();
        let output: Vec<String> = ["fdgacbe", "cefdb", "cefbgd", "gcbe"]
            .iter()
            .map(|e| e.to_string())
            .collect();
        let expected = DisplayPattern { input, output };
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
}
