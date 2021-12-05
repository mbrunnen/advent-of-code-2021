use crate::utils::challenge::Challenge;
use std::cmp::Ordering;

pub struct Day1 {
    data: Vec<u32>,
}

impl Challenge for Day1 {
    fn new(input_file: &str) -> Self {
        let lines: Vec<String> = Self::load(input_file).unwrap();

        Self {
            data: lines
                .iter()
                .map(|l| l.parse::<u32>().expect("Could not parse line"))
                .collect(),
        }
    }

    fn run(&self, part: u32) -> Result<String, String> {
        match part {
            1 => self.run_part_one(),
            x => unimplemented!(
                "Invalid part {} for Day {}",
                x,
                std::any::type_name::<Self>()
            ),
        }
    }
}

impl Day1 {
    fn count_increments<'a, T: Iterator<Item = &'a u32>>(data: T) -> u32 {
        let mut last: Option<u32> = None;

        let check = |mut acc: u32, item: &u32| {
            acc = match last {
                Some(x) => match item.cmp(&x) {
                    Ordering::Greater => {
                        print!("{} (increased)", item);
                        acc + 1
                    }
                    Ordering::Less => {
                        print!("{} (decreased)", item);
                        acc
                    }
                    Ordering::Equal => {
                        print!("{} (no change)", item);
                        acc
                    }
                },
                None => {
                    print!("{} (N/A - previous measurement)", item);
                    0
                }
            };
            last = Some(*item);
            println!(" - {}", acc);
            acc
        };
        data.fold(0, check)
    }

    fn run_part_one(&self) -> Result<String, String> {
        let increased = Self::count_increments(self.data.iter());
        Ok(format!("{:#?}", increased))
    }
}
