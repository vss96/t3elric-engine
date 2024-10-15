use crate::parser::{BoardState, Cell, Player};

use super::Evaluator;

pub struct ColumnEvaluator;

impl Evaluator for ColumnEvaluator {
    fn score(&self, board_state: &BoardState, x: usize, y: usize, player: &Player) -> f32 {
        let rows = board_state.board.get_rows();
        let mut score  = 0;
        let mut empty_space = 1;
        for i in (0..x).rev() {
            if rows[i][y] == Cell::Playable {
                empty_space += 1;
                continue;
            }

            if rows[i][y] == Cell::Played(player.clone()) {
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

            if rows[i][y] == Cell::Played(player.clone()) {
                score += 1
            } else {
                break;
            }
        }

        if score + empty_space < board_state.win_length {
            score = 0;
        }

        score as f32 / empty_space as f32
    }
}
