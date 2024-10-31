
use crate::{
    evaluator::{
        Evaluator, GreedyEvaluator,
    },
    parser::Cell,
};

use super::Scorer;

pub struct GreedyScorer {
    evaluator: Box<dyn Evaluator>,
}

impl Default for GreedyScorer {
    fn default() -> Self {
        Self {
            evaluator: Box::new(GreedyEvaluator::default()),
        }
    }
}

impl Scorer for GreedyScorer {
    fn score(&self, board_state: &mut crate::parser::BoardState) -> f32 {
        if let Some(player) = board_state.get_winner() {
            return 10000f32 * f32::from(player as i16);
        }

        let rows = board_state.board.get_rows();
        let (rlen, clen) = (rows.len(), rows[0].len());
        let mut cum_score = 0f32;
        let player = board_state.player_to_move;
        for i in 0..rlen {
            for j in 0..clen {
                let cell = rows[i][j];

                if cell != Cell::Playable {
                    continue;
                }

                let player_score =
                    (self.evaluator.score(&board_state, i, j, player)) * (f32::from(player as i16));

                cum_score += player_score;
            }
        }
        cum_score
    }
}

#[cfg(test)]
mod test_greedy_scorer {
    use crate::{
        parser::{
            BestMove, Board, BoardState,
            Cell::{Playable, Played},
            Player, TimeSetting,
        },
        scorer::Scorer,
        solver::Solver,
    };

    use super::GreedyScorer;

    #[test]
    fn should_return_positive_score() {
        let mut board_state = BoardState::new(
            Player::X,
            Board::new(vec![
                vec![Played(Player::O), Playable, Playable],
                vec![Playable, Played(Player::O), Played(Player::O)],
                vec![Playable, Played(Player::X), Played(Player::X)],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let score = GreedyScorer::default().score(&mut board_state);
        assert!(score > 0f32);
    }

    #[test]
    fn should_return_negative_score() {
        let mut board_state = BoardState::new(
            Player::O,
            Board::new(vec![
                vec![Playable, Playable, Playable],
                vec![Played(Player::X), Played(Player::X), Played(Player::O)],
                vec![Playable, Playable, Played(Player::O)],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let score = GreedyScorer::default().score(&mut board_state);
        assert!(score < -3000f32);
    }

    #[test]
    fn should_return_negative_score_for_game_where_o_already_won() {
        let mut board_state = BoardState::new(
            Player::X,
            Board::new(vec![
                vec![Playable, Playable, Played(Player::O)],
                vec![Played(Player::X), Played(Player::X), Played(Player::O)],
                vec![Playable, Playable, Played(Player::O)],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let score = GreedyScorer::default().score(&mut board_state);
        assert!(score < -3000f32);
    }

    // w / max((w - s), 1)
}
