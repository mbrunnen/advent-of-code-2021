use crate::utils::challenge::Challenge;

pub struct Day1 {
    data: Vec<u32>,
}

impl Challenge<u32> for Day1 {
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

impl Day1 {
    fn count_increments(data: &[u32]) -> usize {
        data.windows(2).filter(|x| x[0] < x[1]).count()
    }

    fn run_part_one(&self) -> Result<String, String> {
        let increased = Self::count_increments(&self.data);
        Ok(format!("{:#?}", increased))
    }

    fn run_part_two(&self) -> Result<String, String> {
        let sums: Vec<u32> = self.data.windows(3).map(|x| x[0] + x[1] + x[2]).collect();
        let increased = Self::count_increments(&sums);

        Ok(format!("{:#?}", increased))
    }
}
