use crate::naive::{find_possibles, recursive_solve};
use crate::{
    board::{positions::N_CELLS, Board},
    naive::SolveState,
};

fn fill_naked_singles(mut board: Board) -> Board {
    for index in 0..N_CELLS {
        let possibles = find_possibles(&board, index);
        if possibles.len() == 1 {
            board.data[index].value = Some(*possibles.iter().next().unwrap());
        }
    }
    board
}

pub fn smart_solve(board: Board) -> SolveState {
    let fb = fill_naked_singles(board);
    recursive_solve(fb)
}
