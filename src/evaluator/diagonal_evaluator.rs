use std::cmp::max;

use crate::parser::{BoardState, Cell, Player};

use super::Evaluator;

pub struct DiagonalEvaluator;

impl Evaluator for DiagonalEvaluator {
    fn score(&self, board_state: &BoardState, x: usize, y: usize, player: &Player) -> i32 {
        let rows = board_state.board.get_rows();
        let c = rows[0].len();
        let mut d1score: i32 = 0;
        let mut d2score: i32 = 0;
        let player_cell = Cell::Played(player.clone());

        let d1intercept = x as i32 - y as i32;
        let d2intercept = x as i32 + y as i32;

        if d1intercept == 0 {
            for i in 0..c {
                if rows[i][i] == player_cell {
                    d1score += 1;
                }
            }
        }

        if d2intercept == c as i32 - 1 {
            for i in 0..c {
                if rows[i][c - 1 - i] == player_cell {
                    d2score += 1;
                }
            }
        }

        max(d1score, d2score)
    }
}
