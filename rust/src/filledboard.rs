use std::{collections::{HashMap, HashSet}, convert::TryInto, io::{self, Read}};

use crate::board::positions::{HOUSES, House, N_CELLS};

#[derive(Clone, Debug, PartialEq, Eq)]
/// An already filled board
pub struct FilledBoard {
    pub(crate) data: [u8; N_CELLS],
}

impl FilledBoard {
    /// Parse a vec of Boards from a given file.
    ///
    ///  Example board:
    /// ```text
    /// B1-1,Bad,incorrect
    /// 6,9,1,4,8,3,5,2,7
    /// 2,1,3,7,9,5,4,6,8
    /// 5,8,7,6,2,4,9,3,1
    /// 3,5,8,2,7,1,6,9,4
    /// 1,2,6,4,3,9,8,7,5
    /// 7,4,9,8,5,6,2,1,3
    /// 4,7,2,9,1,8,3,5,6
    /// 8,3,1,5,6,2,7,4,9
    /// 9,6,5,3,4,7,1,8,2
    /// ```
    pub fn from_buf(reader: &mut impl Read) -> io::Result<Vec<Self>> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;

        Ok(buf
            .split("\n\n")
            .map(|s| s.trim())
            .filter(|section| !section.trim().is_empty())
            .map(|section| {
                let (_header, board_str) = section.split_at(section.find('\n').unwrap());
                Self::from_str(board_str).unwrap()
            })
            .collect())
    }

    /// Parse a Board from a &str representation
    pub fn from_str(repr: &str) -> Result<Self, String> {
        Ok(Self {
            data: repr
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .or_else(|e: Vec<_>| {
                    Err(format!(
                        "Expected {} numbers in board, got {}",
                        N_CELLS,
                        e.len()
                    ))
                })?,
        })
    }
}

impl FilledBoard {
    /// Check if board is correctly solved
    pub fn is_solved(&self) -> bool {
        HOUSES.iter().all(|rule| self.is_valid_on_rule(rule))
    }

    /// Checks that the board is valid against a certain `Rule`, such as for a row, with rule indeces [0, 1, 2, 3, 4, 5, 6, 7, 8]
    pub fn is_valid_on_rule(&self, rule: &House) -> bool {
        let nums: Vec<u8> = rule.iter().map(|pos| self.data[*pos]).collect();
        // should sum up to a constant; we assume that values are always in [1, 9]
        nums.iter().cloned().fold(0, u8::wrapping_add) == 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 &&
        // should not repeat
        {
            let mut seen = HashSet::with_capacity(9);
            nums.iter().all(|x| seen.insert(x))
        }
    }

    /// Returns a map of value in [1, 9] to the board positions that contain the value in rule
    pub fn get_distr(&self, rule: &House) -> HashMap<u8, Vec<usize>> {
        let mut distr: HashMap<u8, Vec<usize>> = HashMap::with_capacity(9);
        for pos in rule.iter() {
            let num = self.data[*pos];
            distr.entry(num).or_insert(Vec::with_capacity(2)).push(*pos);
        }
        distr
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    pub type HouseGroup = [House; 9];

    use super::*;

    /// Rule for a block, in horizontal order
    const BLOCKS: HouseGroup = [
        [0, 1, 2, 9, 10, 11, 18, 19, 20],
        [3, 4, 5, 12, 13, 14, 21, 22, 23],
        [6, 7, 8, 15, 16, 17, 24, 25, 26],
        [27, 28, 29, 36, 37, 38, 45, 46, 47],
        [30, 31, 32, 39, 40, 41, 48, 49, 50],
        [33, 34, 35, 42, 43, 44, 51, 52, 53],
        [54, 55, 56, 63, 64, 65, 72, 73, 74],
        [57, 58, 59, 66, 67, 68, 75, 76, 77],
        [60, 61, 62, 69, 70, 71, 78, 79, 80],
    ];

    const ROWS: HouseGroup = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [9, 10, 11, 12, 13, 14, 15, 16, 17],
        [18, 19, 20, 21, 22, 23, 24, 25, 26],
        [27, 28, 29, 30, 31, 32, 33, 34, 35],
        [36, 37, 38, 39, 40, 41, 42, 43, 44],
        [45, 46, 47, 48, 49, 50, 51, 52, 53],
        [54, 55, 56, 57, 58, 59, 60, 61, 62],
        [63, 64, 65, 66, 67, 68, 69, 70, 71],
        [72, 73, 74, 75, 76, 77, 78, 79, 80],
    ];

    const COLS: HouseGroup = [
        [0, 9, 18, 27, 36, 45, 54, 63, 72],
        [1, 10, 19, 28, 37, 46, 55, 64, 73],
        [2, 11, 20, 29, 38, 47, 56, 65, 74],
        [3, 12, 21, 30, 39, 48, 57, 66, 75],
        [4, 13, 22, 31, 40, 49, 58, 67, 76],
        [5, 14, 23, 32, 41, 50, 59, 68, 77],
        [6, 15, 24, 33, 42, 51, 60, 69, 78],
        [7, 16, 25, 34, 43, 52, 61, 70, 79],
        [8, 17, 26, 35, 44, 53, 62, 71, 80],
    ];

    const SWAPPED_BOARD0: FilledBoard = FilledBoard {
        #[rustfmt::skip]
        data: [
            6,9,1,4,8,3,5,2,7,
            2,1,3,7,9,5,4,6,8,
            5,8,7,6,2,4,9,3,1,
            3,5,8,2,7,1,6,9,4,
            1,2,6,4,3,9,8,7,5,
            7,4,9,8,5,6,2,1,3,
            4,7,2,9,1,8,3,5,6,
            8,3,1,5,6,2,7,4,9,
            9,6,5,3,4,7,1,8,2,
        ],
    };

    const SWAPPED_BOARD1: FilledBoard = FilledBoard {
        #[rustfmt::skip]
        data: [
            3,5,6,2,9,4,8,7,1,
            1,7,8,6,5,3,4,9,2,
            4,2,9,7,8,1,3,6,5,
            8,1,2,5,4,9,6,3,8,
            6,9,5,1,3,7,2,4,7,
            7,3,4,8,6,2,1,5,9,
            2,6,3,9,7,8,5,1,4,
            9,4,1,3,2,5,7,8,6,
            5,8,7,4,1,6,9,2,3,
        ],
    };

    #[test]
    fn test_parse_single_board() {
        assert_eq!(
            SWAPPED_BOARD0,
            FilledBoard::from_str(
                "6,9,1,4,8,3,5,2,7
        2,1,3,7,9,5,4,6,8
        5,8,7,6,2,4,9,3,1
        3,5,8,2,7,1,6,9,4
        1,2,6,4,3,9,8,7,5
        7,4,9,8,5,6,2,1,3
        4,7,2,9,1,8,3,5,6
        8,3,1,5,6,2,7,4,9
        9,6,5,3,4,7,1,8,2"
            )
            .unwrap()
        );
        assert_eq!(
            SWAPPED_BOARD1,
            FilledBoard::from_str(
                "3,5,6,2,9,4,8,7,1
        1,7,8,6,5,3,4,9,2
        4,2,9,7,8,1,3,6,5
        8,1,2,5,4,9,6,3,8
        6,9,5,1,3,7,2,4,7
        7,3,4,8,6,2,1,5,9
        2,6,3,9,7,8,5,1,4
        9,4,1,3,2,5,7,8,6
        5,8,7,4,1,6,9,2,3"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_parse_board_file() -> io::Result<()> {
        let mut board_file_string = Cursor::new(String::from(
            "B1-1,Bad,incorrect
6,9,1,4,8,3,5,2,7
2,1,3,7,9,5,4,6,8
5,8,7,6,2,4,9,3,1
3,5,8,2,7,1,6,9,4
1,2,6,4,3,9,8,7,5
7,4,9,8,5,6,2,1,3
4,7,2,9,1,8,3,5,6
8,3,1,5,6,2,7,4,9
9,6,5,3,4,7,1,8,2

B3-1,UGLY!,incorrect
3,5,6,2,9,4,8,7,1
1,7,8,6,5,3,4,9,2
4,2,9,7,8,1,3,6,5
8,1,2,5,4,9,6,3,8
6,9,5,1,3,7,2,4,7
7,3,4,8,6,2,1,5,9
2,6,3,9,7,8,5,1,4
9,4,1,3,2,5,7,8,6
5,8,7,4,1,6,9,2,3
",
        ));

        let boards = FilledBoard::from_buf(&mut board_file_string)?;

        assert_eq!(2, boards.len());
        assert_eq!(SWAPPED_BOARD0, boards[0]);
        assert_eq!(SWAPPED_BOARD1, boards[1]);
        Ok(())
    }

    #[test]
    fn valid_rule_group() {
        let b = SWAPPED_BOARD0;
        assert!(ROWS.iter().all(|row| b.is_valid_on_rule(row)));
        assert!((2..9)
            .map(|i| BLOCKS[i])
            .all(|rule| b.is_valid_on_rule(&rule)));
        assert!((0..2)
            .chain(4..9)
            .map(|i| COLS[i])
            .all(|rule| b.is_valid_on_rule(&rule)));

        assert!(![0, 1]
            .iter()
            .map(|i| BLOCKS[*i])
            .all(|rule| b.is_valid_on_rule(&rule)));
        assert!(![2, 3]
            .iter()
            .map(|i| COLS[*i])
            .all(|rule| b.is_valid_on_rule(&rule)));
    }
}
