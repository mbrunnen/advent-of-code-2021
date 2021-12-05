use crate::utils::challenge::Challenge;

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

    fn run(&self) -> Result<String, String> {
        let mut last: Option<u32> = None;

        let increased = self.data.iter().fold(0, |mut acc, item| {
            acc = match last {
                Some(x) => {
                    if item > &x {
                        print!("{} (increased)", item);
                        acc + 1
                    } else if item < &x {
                        print!("{} (decreased)", item);
                        acc
                    } else {
                        print!("{} (no change)", item);
                        acc
                    }
                }
                None => {
                    print!("{} (N/A - previous measurement)", item);
                    0
                }
            };
            last = Some(*item);
            println!(" - {}", acc);
            acc
        });

        Ok(format!("{:#?}", increased))
    }
}
