use std::cmp::max;

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
        rows: &Vec<Vec<crate::parser::Cell>>,
        x: usize,
        y: usize,
        player: &crate::parser::Player,
    ) -> i32 {
        let s = self.evaluator.score(rows, x, y, player);
        let rlen = rows.len();
        rlen as i32 / max(rlen as i32 - s, 1)
    }
}
