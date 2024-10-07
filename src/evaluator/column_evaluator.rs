use crate::parser::{BoardState, Cell, Player};

use super::Evaluator;

pub struct ColumnEvaluator;

impl Evaluator for ColumnEvaluator {
    fn score(&self, board_state: &BoardState, x: usize, y: usize, player: &Player) -> i32 {
        board_state
        .board
        .get_rows()
        .iter()
            .filter(|c| c[y] == Cell::Played(player.clone()))
            .count() as i32
    }
}
