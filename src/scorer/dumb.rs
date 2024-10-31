use crate::parser::{Cell, Player};

use super::Scorer;

struct DumbScorer;

impl Scorer for DumbScorer {
    fn score(&self, board_state: &mut crate::parser::BoardState) -> f32 {
        0f32
    }
}

fn evaluate_line<const N: usize>(line: &[Cell], player: Player, win_length: usize) -> f32 {
    let mut player_count = 0;
    let mut opponent_count = 0;

    for cell in line {
        match cell {
            Cell::Played(p) if *p == player => player_count += 1,
            Cell::Played(p) if *p == player.opponent() => opponent_count += 1,
            _ => {}
        }
    }

    // Scoring heuristic based on counts in line relative to win length
    match (player_count, opponent_count) {
        (n, 0) if n == win_length => 100.0,     // Winning configuration
        (n, 0) if n == win_length - 1 => 50.0,  // One short of winning
        (n, 0) if n == win_length - 2 => 10.0,  // Two short of winning
        (n, 0) if n == 1 => 1.0,                // One in a row
        (0, n) if n == win_length => -100.0,    // Opponent win potential
        (0, n) if n == win_length - 1 => -50.0, // Opponent one short of winning
        (0, n) if n == win_length - 2 => -10.0, // Opponent two short of winning
        (0, 1) => -1.0,                         // Opponent one in a row
        _ => 0.0,                               // Neutral or blocked line
    }
}
