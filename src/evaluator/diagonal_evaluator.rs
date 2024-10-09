use std::cmp::max;

use crate::parser::{BoardState, Cell, Player};

use super::Evaluator;

pub struct DiagonalEvaluator;

impl Evaluator for DiagonalEvaluator {
    fn score(&self, board_state: &BoardState, x: usize, y: usize, player: &Player) -> i32 {
        let rows = board_state.board.get_rows();
        let row_count = rows.len();  // Number of rows (M)
        let col_count = rows[0].len();  // Number of columns (M)
        let mut d1score: i32 = 0;
        let mut d2score: i32 = 0;
        let player_cell = Cell::Played(player.clone());

        // Diagonal 1: slope +1 (top-left to bottom-right), y = x + (y - x)
        let mut row = if x > y { x - y } else { 0 };
        let mut col = if y > x { y - x } else { 0 };

        while row < row_count && col < col_count {
            if rows[row][col] == player_cell {
                d1score += 1;
            }
            row += 1;
            col += 1;
        }

        // Diagonal 2: slope -1 (top-right to bottom-left), y = -x + (x + y)
        let mut row = if x + y < col_count { 0 } else { (x + y) - (col_count - 1) };
        let mut col = if x + y < col_count { x + y } else { col_count - 1 };

        while row < row_count && col < col_count && col >= 0 {
            if rows[row][col] == player_cell {
                d2score += 1;
            }
            row += 1;
            if col == 0 {
                break;
            }
            col -= 1;
        }

        max(d1score, d2score)


    }
}

/*
0 0 0 
0 0 0
0 0 0
*/