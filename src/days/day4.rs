use crate::utils::challenge::Challenge;
use std::collections::HashMap;
use std::thread;

pub struct Day4 {
    data: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct BingoBoard {
    data: HashMap<u32, (usize, usize)>,
    row_cnt: Vec<u32>,
    col_cnt: Vec<u32>,
}

impl BingoBoard {
    fn check(&mut self, num: &u32) -> bool {
        if self.data.contains_key(num) {
            let (row, col) = self.data.remove(num).unwrap();
            self.row_cnt[row] += 1;
            self.col_cnt[col] += 1;
        }

        self.row_cnt.iter().any(|&x| x > 4) || self.col_cnt.iter().any(|&x| x > 4)
    }

    fn play(&mut self, input: &[u32]) -> Option<(usize, u32, u32)> {
        for (i, v) in input.iter().enumerate() {
            if self.data.contains_key(v) && self.check(v) {
                let sum_unmarked: u32 = self.data.iter().map(|(key, _)| *key).sum();
                return Some((i, *v, sum_unmarked));
            }
        }

        None
    }
}

impl From<&[String]> for BingoBoard {
    fn from(input: &[String]) -> Self {
        let mut data: HashMap<u32, (usize, usize)> = HashMap::new();
        for (i, l) in input.iter().enumerate() {
            data.extend(
                l.split(' ')
                    .filter(|s| !s.is_empty())
                    .enumerate()
                    .map(|(j, s)| (s.parse::<u32>().unwrap(), (i, j)))
                    .collect::<HashMap<u32, (usize, usize)>>(),
            )
        }

        BingoBoard {
            data,
            row_cnt: vec![0, 0, 0, 0, 0],
            col_cnt: vec![0, 0, 0, 0, 0],
        }
    }
}

#[derive(Debug, PartialEq)]
struct BingoSubsystem {
    input: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl From<&[String]> for BingoSubsystem {
    fn from(data: &[String]) -> Self {
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

impl BingoSubsystem {
    fn play(mut self) -> (u32, u32) {
        let handles: Vec<_> = (0..self.boards.len())
            .map(|_| {
                let mut board = self.boards.pop().unwrap();
                let input = self.input.clone();

                thread::spawn(move || board.play(&input))
            })
            .collect();

        let mut winner = self.input.len();
        let mut loser = 0;

        let mut winner_result = 0;
        let mut loser_result = 0;

        for h in handles {
            if let Some((idx, last, sum_unmarked)) = h.join().unwrap() {
                if idx < winner {
                    winner = idx;
                    winner_result = last * sum_unmarked;
                }
                if idx > loser {
                    loser = idx;
                    loser_result = last * sum_unmarked;
                }
            }
        }

        (winner_result, loser_result)
    }
}

impl Challenge <String>for Day4 {
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
        let bingo = BingoSubsystem::from(&self.data[..]);
        let (result, _) = bingo.play();
        Ok(format!("{:#?}", result))
    }

    fn run_part_two(&self) -> Result<String, String> {
        let bingo = BingoSubsystem::from(&self.data[..]);
        let (_, result) = bingo.play();
        Ok(format!("{:#?}", result))
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
        let expected: HashMap<u32, (usize, usize)> = HashMap::from([
            (3, (3, 2)),
            (17, (0, 2)),
            (11, (0, 3)),
            (8, (1, 0)),
            (24, (1, 4)),
            (0, (0, 4)),
            (4, (1, 3)),
            (9, (2, 1)),
            (15, (4, 3)),
            (20, (4, 2)),
            (21, (2, 0)),
            (12, (4, 1)),
            (16, (2, 3)),
            (22, (0, 0)),
            (10, (3, 1)),
            (6, (3, 0)),
            (23, (1, 2)),
            (14, (2, 2)),
            (2, (1, 1)),
            (19, (4, 4)),
            (1, (4, 0)),
            (13, (0, 1)),
            (7, (2, 4)),
            (18, (3, 3)),
            (5, (3, 4)),
        ]);

        let bingo = BingoBoard::from(&get_input()[2..7]);
        assert_eq!(bingo.data, expected);
    }

    #[test]
    fn test_bingo_subsystem_from() {
        let expected_input = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let expected_boards: Vec<BingoBoard> = vec![
            BingoBoard::from(&get_input()[2..7]),
            BingoBoard::from(&get_input()[8..13]),
            BingoBoard::from(&get_input()[14..19]),
        ];
        let bingo = BingoSubsystem::from(&get_input()[..]);

        assert_eq!(bingo.input, expected_input);
        assert_eq!(bingo.boards, expected_boards);
    }

    #[test]
    fn test_bingo_subsystem_play() {
        let bingo = BingoSubsystem::from(&get_input()[..]);
        let (win, lose) = bingo.play();
        assert_eq!(win, 4512);
        assert_eq!(lose, 1924);
    }
}
