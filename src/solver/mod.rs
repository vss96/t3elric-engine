use crate::parser::{BestMove, BoardState};

mod first_move_solver;
mod greedy_solver;

pub use greedy_solver::GreedySolver;

pub trait Solver {
    fn solve(board_state: &BoardState) -> Option<BestMove>;
}
