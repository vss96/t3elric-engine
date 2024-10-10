use crate::parser::{BoardState, Cell, Player};

use super::Evaluator;

pub struct RowEvaluator;

impl Evaluator for RowEvaluator {
    fn score(&self, board_state: &BoardState, x: usize, y: usize, player: &Player) -> i32 {
        let row = board_state.board.get_rows()[x as usize].clone();

        let mut score: i32= 0;
        let mut empty_space = 1;
        for i in (0..y).rev() {

            if row[i] == Cell::Playable {
                empty_space += 1;
                continue;
            }

            if row[i] == Cell::Played(player.clone()) {
                score += 1
            } else {
                break;
            }
        }

        for i in y + 1..row.len() {

            if row[i] == Cell::Playable {
                empty_space += 1;
                continue;
            }

            if row[i] == Cell::Played(player.clone()) {
                score += 1
            } else {
                break;
            }
        }

        if score + empty_space < board_state.win_length as i32 {
            score = 0;
        }

        score
    }
}
