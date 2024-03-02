use crate::parser::{Cell, Player};

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
    fn score(&self, rows: &Vec<Vec<Cell>>, x: usize, y: usize, player: &Player) -> i32 {
        self.evaluator.score(rows, x, y, &player.opponent())
    }
}
