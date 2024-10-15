use crate::parser::BoardState;

mod greedy;

pub trait Scorer {

    fn score(&self, board_state: &BoardState)  -> f32;
}

pub use greedy::GreedyScorer;