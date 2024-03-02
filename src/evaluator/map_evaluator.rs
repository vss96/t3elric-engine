use super::Evaluator;

pub struct MapEvaluator {
    evaluator: Box<dyn Evaluator>,
    mapper: Box<dyn Fn(i32) -> i32>,
}

impl MapEvaluator {
    pub fn new(evaluator: Box<dyn Evaluator>, mapper: Box<dyn Fn(i32) -> i32>) -> Self {
        Self { evaluator, mapper }
    }
}

impl Evaluator for MapEvaluator {
    fn score(
        &self,
        rows: &Vec<Vec<crate::parser::Cell>>,
        x: usize,
        y: usize,
        player: &crate::parser::Player,
    ) -> i32 {
        (self.mapper)(self.evaluator.score(rows, x, y, player))
    }
}
