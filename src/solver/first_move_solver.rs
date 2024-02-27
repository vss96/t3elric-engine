use crate::parser::{BestMove, BoardState, Cell};

use super::Solver;

pub struct FirstMoveSolver;

impl Solver for FirstMoveSolver {
    fn solve(board_state: &BoardState) -> Option<crate::parser::BestMove> {
        let rows = &board_state.board.get_rows();
        // println!("rows: {:?}", rows);
        let (mut x, mut y) = (0, 0);
        for row in rows {
            for cell in row {
                if *cell == Cell::Playable {
                    // println!("move {} {}", x, y);
                    return Some(BestMove::new(y, x));
                }
                y += 1;
            }
            x += 1;
            y = 0;
        }
        None
    }
}
