use super::Evaluator;

pub struct ReduceEvaluator {
    evaluators: Vec<Box<dyn Evaluator>>,
    reducer: Box<dyn Fn(i32, i32) -> i32>,
}

impl ReduceEvaluator {
    pub fn new(evaluators: Vec<Box<dyn Evaluator>>, reducer: Box<dyn Fn(i32, i32) -> i32>) -> Self {
        Self {
            evaluators,
            reducer,
        }
    }
}

impl Evaluator for ReduceEvaluator {
    fn score(
        &self,
        rows: &Vec<Vec<crate::parser::Cell>>,
        x: usize,
        y: usize,
        player: &crate::parser::Player,
    ) -> i32 {
        self.evaluators
            .iter()
            .map(|f| f.score(rows, x, y, player))
            .fold(0, |acc, x| (self.reducer)(acc, x))
    }
}
