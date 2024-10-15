use crate::{evaluator::{ColumnEvaluator, DiagonalEvaluator, Evaluator, GreedyEvaluator, ReduceEvaluator, RowEvaluator, WinningEvaluator}, parser::Player};

use super::Scorer;

pub struct GreedyScorer {
    evaluator: Box<dyn Evaluator>,
    win_evaluator: Box<dyn Evaluator>
}

impl Default for GreedyScorer {
    fn default() -> Self {
        let win_evaluators: Vec<Box<dyn Evaluator>> = vec![
            Box::new(WinningEvaluator::new(Box::new(ColumnEvaluator))),
            Box::new(WinningEvaluator::new(Box::new(RowEvaluator))),
            Box::new(WinningEvaluator::new(Box::new(DiagonalEvaluator))),
        ];
        let winning_evaluator: Box<dyn Evaluator> = Box::new(ReduceEvaluator::new(
            win_evaluators,
            Box::new(|x, y| f32::max(x, y)),
        ));
        Self {
            evaluator: Box::new(GreedyEvaluator::default()),
            win_evaluator: winning_evaluator
        }
    }
}

impl Scorer for GreedyScorer {
    fn score(&self, board_state: &crate::parser::BoardState) -> f32 {
        let rows = &board_state.board.get_rows();
        let (rlen, clen) = (rows.len(), rows[0].len());
        let mut cum_score = 0f32;
        for i in 0..rlen {
            for j in 0..clen {
                let x_win_score = self.win_evaluator.score(&board_state, i, j, &Player::X);
                let o_win_score = self.win_evaluator.score(&board_state, i, j, &Player::O);


                if x_win_score == board_state.win_length as f32 {
                    cum_score += 1000f32;
                    break;
                }

                if o_win_score == board_state.win_length as f32 {
                    cum_score -= 1000f32;
                    break;
                }

                cum_score += self.evaluator.score(&board_state, i, j, &Player::X);
                cum_score -= self.evaluator.score(&board_state, i, j, &Player::O);

            }
        }
        cum_score
    }
}
