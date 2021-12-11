use crate::utils::challenge::Challenge;

pub struct Day6 {
    data: Vec<String>,
}

impl Challenge for Day6 {
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

impl Day6 {
    fn run_part_one(&self) -> Result<String, String> {
        Ok(format!("{:#?}", 0))
    }

    fn run_part_two(&self) -> Result<String, String> {
        Ok(format!("{:#?}", 0))
    }
}
