use crate::parser::{BoardState, Cell, Player};

use super::Evaluator;

pub struct RowEvaluator;

impl Evaluator for RowEvaluator {
    fn score(&self, board_state: &BoardState, x: usize, y: usize, player: &Player) -> i32 {
        board_state.board.get_rows()[x as usize]
            .iter()
            .filter(|c| **c == Cell::Played(player.clone()))
            .count() as i32
    }
}
