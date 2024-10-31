use crate::parser::{BestMove, BoardState};

mod first_move_solver;
mod greedy_solver;
mod look_ahead_solver;

pub use greedy_solver::GreedySolver;
pub use look_ahead_solver::LookAheadSolver;

pub trait Solver {
    fn solve(&self, board_state: &mut BoardState) -> (Option<BestMove>, f32);
}
