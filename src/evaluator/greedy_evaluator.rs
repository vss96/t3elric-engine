use crate::evaluator::Evaluator;

use super::{
    ColumnEvaluator, DiagonalEvaluator, MapEvaluator, OpponentEvaluator, ReduceEvaluator,
    RowEvaluator, WinningEvaluator,
};

pub struct GreedyEvaluator {
    evaluator: Box<dyn Evaluator>,
}

impl Default for GreedyEvaluator {
    fn default() -> Self {
        let win_evaluators: Vec<Box<dyn Evaluator>> = vec![
            Box::new(WinningEvaluator::new(Box::new(ColumnEvaluator))),
            Box::new(WinningEvaluator::new(Box::new(RowEvaluator))),
            Box::new(WinningEvaluator::new(Box::new(DiagonalEvaluator))),
        ];

        let losing_evaluators: Vec<Box<dyn Evaluator>> = vec![
            Box::new(WinningEvaluator::new(Box::new(OpponentEvaluator::new(
                Box::new(ColumnEvaluator),
            )))),
            Box::new(WinningEvaluator::new(Box::new(OpponentEvaluator::new(
                Box::new(RowEvaluator),
            )))),
            Box::new(WinningEvaluator::new(Box::new(OpponentEvaluator::new(
                Box::new(DiagonalEvaluator),
            )))),
        ];

        let winning_evaluator: Box<dyn Evaluator> = Box::new(ReduceEvaluator::new(
            win_evaluators,
            Box::new(|x, y| f32::max(x, y)),
        ));
        let losing_evaluator: Box<dyn Evaluator> = Box::new(MapEvaluator::new(
            Box::new(ReduceEvaluator::new(
                losing_evaluators,
                Box::new(|x, y| f32::max(x, y)),
            )),
            Box::new(|f| f * 1.0),
        ));

        let player_score_evaluator = Box::new(ReduceEvaluator::new(
            vec![
                Box::new(ColumnEvaluator),
                Box::new(RowEvaluator),
                Box::new(DiagonalEvaluator),
            ],
            Box::new(|x, y| f32::max(x, y)),
        ));

        let player_sum_evaluator = Box::new(ReduceEvaluator::new(
            vec![
                Box::new(ColumnEvaluator),
                Box::new(RowEvaluator),
                Box::new(DiagonalEvaluator),
            ],
            Box::new(|x, y| x + y),
        ));

        let greedy_evaluator = Box::new(ReduceEvaluator::new(
            vec![
                winning_evaluator,
                losing_evaluator,
                player_score_evaluator,
                player_sum_evaluator,
            ],
            Box::new(|x, y| f32::max(x, y)),
        ));
        Self {
            evaluator: greedy_evaluator,
        }
    }
}

impl Evaluator for GreedyEvaluator {
    fn score(
        &self,
        board: &crate::parser::BoardState,
        x: usize,
        y: usize,
        player: &crate::parser::Player,
    ) -> f32 {
        self.evaluator.score(board, x, y, player)
    }
}
