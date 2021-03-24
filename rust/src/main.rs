use std::{fs::File, io, time::Instant};

use rust::{board::parse_board_list, naive::recursive_solve};

fn main() -> io::Result<()> {
    let boards = parse_board_list(&mut File::open("unsolved.txt")?).expect("error parsing boards");

    for board in boards.into_iter().skip(3) {
        let start = Instant::now();
        let res = recursive_solve(board.board);
        println!(
            "Board {} took {} backtracks ({:#?} millisec)",
            board.id,
            res.nbacktracks,
            (Instant::now() - start).as_millis()
        );
        println!("{}", res.solved.unwrap());
    }

    Ok(())
}
