use crate::parser::{BoardState, Player};

use super::Evaluator;

pub struct OpponentEvaluator {
    evaluator: Box<dyn Evaluator>,
}

impl OpponentEvaluator {
    pub fn new(evaluator: Box<dyn Evaluator>) -> Self {
        Self { evaluator }
    }
}

impl Evaluator for OpponentEvaluator {
    fn score(&self, board_state: &BoardState, x: usize, y: usize, player: &Player) -> i32 {
        self.evaluator.score(board_state, x, y, &player.opponent())
    }
}
