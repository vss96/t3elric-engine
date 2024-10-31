
use crate::parser::{BoardState, Cell, Player};

use super::Evaluator;

pub struct ColumnEvaluator;

impl Evaluator for ColumnEvaluator {
    fn score(&self, board_state: &BoardState, x: usize, y: usize, player: Player) -> f32 {
        let rows = board_state.board.get_rows();
        let mut score  = 0;
        let mut empty_space = 1;
        for i in (0..x).rev() {
            if rows[i][y] == Cell::Playable {
                empty_space += 1;
                continue;
            }

            if rows[i][y] == Cell::Played(player) {
                score += 1
            } else {
                break;
            }
        }

        for i in x + 1..rows.len() {
            if rows[i][y] == Cell::Playable {
                empty_space +=1;
                continue;
            }

            if rows[i][y] == Cell::Played(player) {
                score += 1
            } else {
                break;
            }
        }

        if score + empty_space < board_state.win_length {
            score = 0;
        }

        // println!("{}, {}", score, empty_space);

        score as f32 
    }
}


#[cfg(test)]
mod test_column_evaluator {
    use crate::{
        evaluator::Evaluator, parser::{
            Board, BoardState,
            Cell::{Playable, Played},
            Player, TimeSetting,
        }

    };

    use super::ColumnEvaluator;

    #[test]
    fn should_score_column_as_2_for_x() {
        let mut board_state = BoardState::new(
            Player::X,
            Board::new(vec![
                vec![Played(Player::O), Playable, Playable],
                vec![Playable, Played(Player::X), Played(Player::O)],
                vec![Playable, Played(Player::X), Played(Player::X)],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let score = ColumnEvaluator.score(&mut board_state, 0, 1, Player::X);
        assert_eq!(score, 2f32);
    }

    #[test]
    fn should_score_column_as_0_for_x() {
        let mut board_state = BoardState::new(
            Player::X,
            Board::new(vec![
                vec![Playable, Playable, Played(Player::O)],
                vec![Playable, Played(Player::X), Playable],
                vec![Playable, Played(Player::X), Played(Player::X)],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let score = ColumnEvaluator.score(&mut board_state, 1, 2, Player::X);
        assert_eq!(score, 1f32);
    }
}