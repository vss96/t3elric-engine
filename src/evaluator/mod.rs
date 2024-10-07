use crate::parser::{BoardState, Player};

pub trait Evaluator {
    fn score(&self, board: &BoardState, x: usize, y: usize, player: &Player) -> i32;
}

mod column_evaluator;
mod diagonal_evaluator;
mod map_evaluator;
mod opponent_evaluator;
mod reduce_evaluator;
mod row_evaluator;
mod winning_evaluator;

pub use column_evaluator::ColumnEvaluator;
pub use diagonal_evaluator::DiagonalEvaluator;
pub use map_evaluator::MapEvaluator;
pub use opponent_evaluator::OpponentEvaluator;
pub use reduce_evaluator::ReduceEvaluator;
pub use row_evaluator::RowEvaluator;
pub use winning_evaluator::WinningEvaluator;
