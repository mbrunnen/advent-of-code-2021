use crate::utils::challenge::Challenge;

pub struct Day4 {
    data: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct BingoBoard {
    data: Vec<Vec<u32>>,
}

impl From<&[String]> for BingoBoard {
    fn from(data: &[String]) -> Self {
        let data: Vec<Vec<u32>> = data
            .iter()
            .map(|l| {
                l.split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        BingoBoard { data }
    }
}

#[derive(Debug, PartialEq)]
struct BingoSubsystem {
    input: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl From<Vec<String>> for BingoSubsystem {
    fn from(data: Vec<String>) -> Self {
        let input: Vec<u32> = data[0]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let boards: Vec<BingoBoard> = (2..data.len())
            .step_by(6)
            .map(|i| BingoBoard::from(&data[i..i + 5]))
            .collect();

        BingoSubsystem { input, boards }
    }
}

impl Challenge for Day4 {
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

impl Day4 {
    fn run_part_one(&self) -> Result<String, String> {
        Ok(format!("{:#?}", 0))
    }

    fn run_part_two(&self) -> Result<String, String> {
        Ok(format!("{:#?}", 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_input() -> Vec<String> {
        vec![
            String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"),
            String::from(""),
            String::from("22 13 17 11  0"),
            String::from("8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from("6 10  3 18  5"),
            String::from("1 12 20 15 19"),
            String::from(""),
            String::from("3 15  0  2 22"),
            String::from("9 18 13 17  5"),
            String::from("19  8  7 25 23"),
            String::from("20 11 10 24  4"),
            String::from("14 21 16 12  6"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from("2  0 12  3  7"),
        ]
    }

    #[test]
    fn test_bingo_board_from() {
        let expected = BingoBoard {
            data: vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ],
        };

        assert_eq!(BingoBoard::from(&get_input()[2..7]), expected);
    }

    #[test]
    fn test_bingo_subsystem_from() {
        let expected = BingoSubsystem {
            input: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                BingoBoard {
                    data: vec![
                        vec![22, 13, 17, 11, 0],
                        vec![8, 2, 23, 4, 24],
                        vec![21, 9, 14, 16, 7],
                        vec![6, 10, 3, 18, 5],
                        vec![1, 12, 20, 15, 19],
                    ],
                },
                BingoBoard {
                    data: vec![
                        vec![3, 15, 0, 2, 22],
                        vec![9, 18, 13, 17, 5],
                        vec![19, 8, 7, 25, 23],
                        vec![20, 11, 10, 24, 4],
                        vec![14, 21, 16, 12, 6],
                    ],
                },
                BingoBoard {
                    data: vec![
                        vec![14, 21, 17, 24, 4],
                        vec![10, 16, 15, 9, 19],
                        vec![18, 8, 23, 26, 20],
                        vec![22, 11, 13, 6, 5],
                        vec![2, 0, 12, 3, 7],
                    ],
                },
            ],
        };

        let bingo = BingoSubsystem::from(get_input());
        println!("{:#?}", bingo);
        assert_eq!(bingo, expected);
    }
}
