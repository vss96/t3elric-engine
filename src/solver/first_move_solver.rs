use crate::parser::{BestMove, BoardState, Cell};

use super::Solver;

pub struct FirstMoveSolver;

impl Solver for FirstMoveSolver {
    fn solve(&self, board_state: &mut BoardState) -> (Option<crate::parser::BestMove>, f32) {
        let rows = &board_state.board.get_rows();
        let (mut x, mut y) = (0, 0);
        for row in rows {
            for cell in row {
                if *cell == Cell::Playable {
                    return (Some(BestMove::new(y, x)), 100.0);
                }
                y += 1;
            }
            x += 1;
            y = 0;
        }
        (None, -1f32)
    }
}
