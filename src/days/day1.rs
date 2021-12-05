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
        Ok(format!("{:#?}", self.data))
    }
}
