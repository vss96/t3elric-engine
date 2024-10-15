use crate::parser::BoardState;

use super::Evaluator;

pub struct MapEvaluator {
    evaluator: Box<dyn Evaluator>,
    mapper: Box<dyn Fn(f32) -> f32>,
}

impl MapEvaluator {
    pub fn new(evaluator: Box<dyn Evaluator>, mapper: Box<dyn Fn(f32) -> f32>) -> Self {
        Self { evaluator, mapper }
    }
}

impl Evaluator for MapEvaluator {
    fn score(
        &self,
        board_state: &BoardState,
        x: usize,
        y: usize,
        player: &crate::parser::Player,
    ) -> f32 {
        (self.mapper)(self.evaluator.score(board_state, x, y, player))
    }
}
