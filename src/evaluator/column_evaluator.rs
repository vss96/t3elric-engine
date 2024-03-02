use crate::parser::{Cell, Player};

use super::Evaluator;

pub struct ColumnEvaluator;

impl Evaluator for ColumnEvaluator {
    fn score(&self, rows: &Vec<Vec<Cell>>, x: usize, y: usize, player: &Player) -> i32 {
        rows.iter()
            .filter(|c| c[y] == Cell::Played(player.clone()))
            .count() as i32
    }
}
