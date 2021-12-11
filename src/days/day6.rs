use crate::utils::challenge::Challenge;

#[derive(Debug, PartialEq)]
struct LanternfishPopulation {
    ages: Vec<usize>,
}

impl From<&str> for LanternfishPopulation {
    fn from(input: &str) -> Self {
        let mut ages: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        let timers: Vec<u32> = input
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        for (i, v) in ages.iter_mut().enumerate() {
            *v = timers.iter().filter(|&v| *v == i as u32).count();
        }

        Self { ages }
    }
}

impl LanternfishPopulation {
    fn simulate_day(&mut self) {
        self.ages = vec![
            self.ages[1],
            self.ages[2],
            self.ages[3],
            self.ages[4],
            self.ages[5],
            self.ages[6],
            self.ages[7] + self.ages[0],
            self.ages[8],
            self.ages[0],
        ];
    }

    fn simulate(&mut self, days: usize) -> usize {
        for _ in 0..days {
            self.simulate_day();
        }

        self.ages.iter().sum()
    }
}

pub struct Day6 {
    data: Vec<String>,
}

impl Challenge <String>for Day6 {
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
        let mut pop = LanternfishPopulation::from(&self.data[0][..]);
        let fishes = pop.simulate(80);
        Ok(format!("{:#?}", fishes))
    }

    fn run_part_two(&self) -> Result<String, String> {
        let mut pop = LanternfishPopulation::from(&self.data[0][..]);
        let fishes = pop.simulate(256);
        Ok(format!("{:#?}", fishes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let expected = LanternfishPopulation {
            ages: vec![0, 1, 1, 2, 1, 0, 0, 0, 0],
        };
        let pop = LanternfishPopulation::from("3,4,3,1,2");
        assert_eq!(expected, pop);
    }

    #[test]
    fn test_simulate_one_day() {
        let expected = LanternfishPopulation {
            ages: vec![1, 1, 2, 1, 0, 0, 0, 0, 0],
        };
        let mut pop = LanternfishPopulation::from("3,4,3,1,2");
        pop.simulate_day();
        assert_eq!(expected, pop);
    }

    #[test]
    fn test_simulate_two_days() {
        let expected = LanternfishPopulation {
            ages: vec![1, 2, 1, 0, 0, 0, 1, 0, 1],
        };
        let mut pop = LanternfishPopulation::from("3,4,3,1,2");
        pop.simulate_day();
        pop.simulate_day();
        assert_eq!(expected, pop);
    }

    #[test]
    fn test_simulate() {
        let mut pop = LanternfishPopulation::from("3,4,3,1,2");
        let fishes = pop.simulate(18);
        assert_eq!(26, fishes);
        let fishes = pop.simulate(80 - 18);
        assert_eq!(5934, fishes);
    }
}
