pub mod positions;

use std::{
    convert::TryInto,
    fmt::{Debug, Display},
    io::{self, Read},
    num::NonZeroU8,
    u8,
};

use fnv::FnvHashSet;

use self::positions::{House, HOUSES, N_CELLS};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub(crate) value: Option<NonZeroU8>,
    // pub(crate) possibles: FnvHashSet<u8>,
    // pub(crate) used_possibilities: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub(crate) data: [Tile; N_CELLS],
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.data.iter().as_slice().chunks_exact(9) {
            for cell in chunk.iter() {
                write!(
                    f,
                    "{},",
                    match cell.value {
                        Some(val) => val.to_string(),
                        None => '_'.to_string(),
                    }
                )?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

pub struct AnnotatedBoard {
    pub id: String,
    pub src: String,
    pub status: String,
    pub board: Board,
}

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
pub fn parse_board_list(reader: &mut impl Read) -> io::Result<Vec<AnnotatedBoard>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    Ok(buf
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|section| !section.trim().is_empty())
        .map(|section| {
            let (header, board_str) = section.split_at(section.find('\n').unwrap());
            let header: Vec<&str> = header.split(",").collect();
            AnnotatedBoard {
                id: header[0].to_owned(),
                src: header[1].to_owned(),
                status: header[2].to_owned(),
                board: Board::from_str(board_str).unwrap(),
            }
        })
        .collect())
}

impl Board {
    /// Parse a Board from a &str representation
    pub fn from_str(repr: &str) -> Result<Self, String> {
        Ok(Self {
            data: repr
                .chars()
                .filter(|c| c.is_ascii_digit() || c == &'_')
                .map(|c| match c.to_digit(10) {
                    Some(num) => Tile {
                        value: Some(NonZeroU8::new(num as u8).unwrap()),
                        // possibles: FnvHashSet::with_capacity_and_hasher(0, Default::default()),
                        // used_possibilities: false,
                    },
                    None => Tile {
                        value: None,
                        // possibles: FnvHashSet::with_hasher(Default::default()),
                        // used_possibilities: false,
                    },
                })
                .collect::<Vec<_>>()
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

    /// Check if all spots on the board is filled
    pub fn is_filled(&self) -> bool {
        self.data.iter().all(|tile| tile.value.is_some())
    }

    /// Check if board is correctly solved
    pub fn is_solved(&self) -> bool {
        HOUSES.iter().all(|rule| self.is_valid_on_house(rule))
    }

    /// Checks that the board is valid against a certain `Rule`, such as for a row, with rule indeces [0, 1, 2, 3, 4, 5, 6, 7, 8]
    pub fn is_valid_on_house(&self, rule: &House) -> bool {
        if self.data.iter().any(|cell| cell.value.is_none()) {
            return false;
        }

        let nums: Vec<_> = rule
            .iter()
            .map(|pos| self.data[*pos].value.unwrap())
            .collect();

        // should sum up to a constant; we assume that values are always in [1, 9]
        nums.iter().map(|u| u8::from(*u)).fold(0, u8::wrapping_add) == 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 &&
            // should not repeat
            {
                let mut seen = FnvHashSet::with_capacity_and_hasher(9, Default::default());
                nums.iter().all(|x| seen.insert(x))
            }
    }
}
