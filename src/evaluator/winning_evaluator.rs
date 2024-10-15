
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
    ) -> f32 {
        let s = self.evaluator.score(board_state, x, y, player);
        let winlength = board_state.win_length;
        winlength as f32 / f32::max(winlength as f32 - s, 1.0)
    }
}
