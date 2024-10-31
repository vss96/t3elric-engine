use crate::parser::BoardState;

mod greedy;
mod dumb;

pub trait Scorer {

    fn score(&self, board_state: &mut BoardState)  -> f32;
}

pub use greedy::GreedyScorer;