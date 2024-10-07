use std::cmp::max;

use crate::parser::BoardState;

use super::Evaluator;

pub struct WinningEvaluator {
    evaluator: Box<dyn Evaluator>,
}

impl WinningEvaluator {
    pub fn new(evaluator: Box<dyn Evaluator>) -> Self {
        Self { evaluator }
    }
}

impl Evaluator for WinningEvaluator {
    fn score(
        &self,
        board_state: &BoardState,
        x: usize,
        y: usize,
        player: &crate::parser::Player,
    ) -> i32 {
        let s = self.evaluator.score(board_state, x, y, player);
        let winlength = board_state.win_length;
        winlength as i32 / max(winlength as i32 - s, 1)
    }
}
