use std::num::NonZeroU8;

use fnv::FnvHashSet;

use crate::board::{positions::NEIGHBORS, Board};

pub(crate) fn find_possibles(board: &Board, index: usize) -> FnvHashSet<NonZeroU8> {
    let seen: FnvHashSet<NonZeroU8> = NEIGHBORS[index]
        .iter()
        .flat_map(|i| board.data[*i as usize].value)
        .collect();
    let possibles = [
        NonZeroU8::new(1).unwrap(),
        NonZeroU8::new(2).unwrap(),
        NonZeroU8::new(3).unwrap(),
        NonZeroU8::new(4).unwrap(),
        NonZeroU8::new(5).unwrap(),
        NonZeroU8::new(6).unwrap(),
        NonZeroU8::new(7).unwrap(),
        NonZeroU8::new(8).unwrap(),
        NonZeroU8::new(9).unwrap(),
    ]
    .iter()
    .cloned()
    .collect::<FnvHashSet<NonZeroU8>>();

    &possibles - &seen
}

impl Board {
    // pub fn naive_solve(&self) -> Option<(Self, u32)> {
    //     let mut nbacktracks = 0;
    //     let mut frontier = vec![];

    //     // let possible_set: HashSet<u8> = (1..=9).into_iter().collect();
    //     match self
    //         .data
    //         .iter()
    //         .enumerate()
    //         .skip_while(|(_, tile)| tile.value.is_some())
    //         .next()
    //     {
    //         Some((first_empty_index, _)) => {
    //             let mut b = self.clone();
    //             b.data[first_empty_index].possibles = find_possibles(&b, first_empty_index);
    //             frontier.push((b, first_empty_index))
    //         }
    //         None => {
    //             // board is filled, cannot find an empty spot
    //             if self.is_solved() {
    //                 return Some((self.clone(), 0));
    //             } else {
    //                 return None;
    //             }
    //         }
    //     }

    //     while let Some((mut board, curr_i)) = frontier.pop() {
    //         for value in board.data[curr_i].possibles.iter() {}
    //         // if board.data[curr_i].value

    //         if let Some((pos, tile)) = board
    //             .data
    //             .iter()
    //             .enumerate()
    //             .skip(curr_i)
    //             .skip_while(|(pos, tile)| tile.value.is_some())
    //             .next()
    //         {
    //             if tile.possibles.is_empty() {
    //                 if tile.used_possibilities {}
    //             }
    //         }
    //     }

    //     None
    // }
}

pub fn recursive_solve(board: Board) -> SolveState {
    // println!("Solving board: \n{}", board);
    if board.is_filled() {
        if board.is_solved() {
            SolveState {
                nbacktracks: 0,
                solved: Some(board),
            }
        } else {
            SolveState {
                nbacktracks: 1,
                solved: None,
            }
        }
    } else {
        if let Some((i, _)) = board
            .data
            .iter()
            .enumerate()
            .skip_while(|(_, tile)| tile.value.is_some())
            .next()
        {
            let possibles = find_possibles(&board, i);
            if possibles.len() == 1 {
                // we are forced to enter this number on tile
                let mut b = board.clone();
                b.data[i].value = Some(*possibles.iter().next().unwrap());
                return recursive_solve(b);
            } else {
                let mut nbacktracks = 0;

                for p in possibles.iter() {
                    let mut b = board.clone();
                    b.data[i].value = Some(*p);
                    let mut res = recursive_solve(b);

                    if res.solved.is_some() {
                        res.nbacktracks += nbacktracks;
                        return res;
                    } else {
                        // board is not solved, so add 1 to backtrack counts
                        nbacktracks += res.nbacktracks + 1;
                        // continue solving
                    }
                }

                // no solution found here, backtrack
                return SolveState {
                    nbacktracks,
                    solved: None,
                };
            }
        } else {
            unreachable!("An unfilled board should have an empty cell");
        }
    }
}
/// Represents a solving state in recursion

#[derive(Debug)]
pub struct SolveState {
    /// Total num backtrack
    pub nbacktracks: u32,
    /// The board, if solved
    pub solved: Option<Board>,
}

#[cfg(test)]
mod tests {

    use crate::board::Board;

    use super::recursive_solve;

    fn board_a1() -> Board {
        Board::from_str(
            "_,_,4,1,_,_,5,2,7
                2,1,3,7,_,_,_,_,_
                _,_,7,6,2,4,_,_,_
                3,5,_,2,7,_,_,_,_
                _,_,_,_,3,_,8,7,5
                _,4,_,_,_,6,_,1,3
                4,7,2,_,1,_,_,5,_
                _,3,1,_,6,2,_,_,9
                9,_,_,_,_,_,1,8,_",
        )
        .unwrap()
    }

    fn board_a2() -> Board {
        Board::from_str(
            "2,_,_,6,_,_,_,_,_
            6,_,_,_,5,1,_,4,_
            _,7,_,_,_,_,_,_,_
            _,_,_,_,3,_,_,1,4
            _,_,5,_,6,_,_,_,_
            _,1,9,_,4,_,_,5,_
            _,_,6,_,_,_,_,2,5
            _,_,_,9,_,_,_,8,_
            8,9,_,_,_,_,4,_,_",
        )
        .unwrap()
    }

    fn board_a3() -> Board {
        Board::from_str(
            "_,_,6,_,9,_,_,_,_
        1,7,_,_,_,3,_,9,_
        _,_,_,7,_,_,_,_,5
        _,_,_,5,_,_,6,_,_
        _,9,_,_,3,_,2,_,_
        _,_,4,_,_,2,1,_,_
        _,_,_,9,7,8,_,_,_
        _,4,_,_,_,5,_,8,_
        _,_,_,_,_,6,_,_,_",
        )
        .unwrap()
    }

    fn solution_a1() -> Board {
        Board::from_str(
            "6,9,4,1,8,3,5,2,7
        2,1,3,7,9,5,4,6,8
        5,8,7,6,2,4,9,3,1
        3,5,8,2,7,1,6,9,4
        1,2,6,4,3,9,8,7,5
        7,4,9,8,5,6,2,1,3
        4,7,2,9,1,8,3,5,6
        8,3,1,5,6,2,7,4,9
        9,6,5,3,4,7,1,8,2",
        )
        .unwrap()
    }

    fn solution_a2() -> Board {
        Board::from_str(
            "2,5,4,6,9,7,8,3,1
            6,8,3,2,5,1,7,4,9
            9,7,1,4,8,3,5,6,2
            7,6,8,5,3,9,2,1,4
            4,2,5,1,6,8,3,9,7
            3,1,9,7,4,2,6,5,8
            1,3,6,8,7,4,9,2,5
            5,4,7,9,2,6,1,8,3
            8,9,2,3,1,5,4,7,6",
        )
        .unwrap()
    }

    fn solution_a3() -> Board {
        Board::from_str(
            "3,5,6,2,9,4,8,7,1
        1,7,8,6,5,3,4,9,2
        4,2,9,7,8,1,3,6,5
        8,1,2,5,4,9,6,3,7
        6,9,5,1,3,7,2,4,8
        7,3,4,8,6,2,1,5,9
        2,6,3,9,7,8,5,1,4
        9,4,1,3,2,5,7,8,6
        5,8,7,4,1,6,9,2,3",
        )
        .unwrap()
    }

    #[test]
    fn test_parse_board() {
        board_a1();
        board_a2();
        board_a3();
    }

    #[test]
    fn test_solve() {
        let res1 = recursive_solve(board_a1());
        let res2 = recursive_solve(board_a2());
        let res3 = recursive_solve(board_a3());
        assert_eq!(solution_a1(), res1.solved.unwrap());
        assert_eq!(solution_a2(), res2.solved.unwrap());
        assert_eq!(solution_a3(), res3.solved.unwrap());

        println!(
            "{},{},{}",
            res1.nbacktracks, res2.nbacktracks, res3.nbacktracks
        );
        println!(
            "Total: {}",
            res1.nbacktracks + res2.nbacktracks + res3.nbacktracks
        );
    }

    #[test]
    fn regress_weird_board() {
        let b = Board::from_str(
            "6,9,4,1,8,3,5,2,7,
        2,1,3,7,5,9,6,4,8,
        5,8,7,6,2,4,9,3,1,
        3,5,9,2,7,1,4,6,_,
        _,_,_,_,3,_,8,7,5,
        _,4,_,_,_,6,_,1,3,
        4,7,2,_,1,_,_,5,_,
        _,3,1,_,6,2,_,_,9,
        9,_,_,_,_,_,1,8,_,",
        )
        .unwrap();
        println!("{:?}", recursive_solve(b));
    }
}
