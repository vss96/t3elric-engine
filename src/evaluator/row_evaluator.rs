use crate::parser::{Cell, Player};

use super::Evaluator;

pub struct RowEvaluator;

impl Evaluator for RowEvaluator {
    fn score(&self, rows: &Vec<Vec<Cell>>, x: usize, y: usize, player: &Player) -> i32 {
        rows[x as usize]
            .iter()
            .filter(|c| **c == Cell::Played(player.clone()))
            .count() as i32
    }
}
