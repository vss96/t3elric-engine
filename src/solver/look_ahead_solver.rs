use crate::{evaluator::{self, Evaluator}, parser::{BestMove, BoardState, Player}, scorer::Scorer};

use super::Solver;

pub struct LookAheadSolver {
    scorer: Box<dyn Scorer>,
    depth: u32,
}

impl LookAheadSolver {
    /// Creates a new LookAheadSolver with the given solver and search depth.
    pub fn new(
        scorer: Box<dyn Scorer>,
        depth: u32) -> Self {
        Self {  scorer, depth }
    }

    /// Alpha-Beta pruning recursive function.
    fn alpha_beta(
        &self,
        board_state: &BoardState,
        depth: u32,
        mut alpha: f32,
        mut beta: f32,
        maximizing_player: bool,
    ) -> (Option<BestMove>, f32) {
        if depth == 0 || board_state.is_game_over() {
            return (None, self.evaluate(board_state));
        }

        let possible_moves = board_state.get_possible_moves();

        // If no possible moves, evaluate the board
        if possible_moves.is_empty() {
            let evaluation = self.evaluate(board_state);
            return (None, evaluation);
        }

        let mut best_move = None;

        if maximizing_player {
            let mut max_eval = f32::NEG_INFINITY;
            for m in possible_moves {
                let new_state = board_state.apply_move(&m);
                let (_, eval) = self.alpha_beta(&new_state, depth - 1, alpha, beta, false);
                if eval > max_eval {
                    max_eval = eval;
                    best_move = Some(m.clone());
                }
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break; // Beta cutoff
                }
            }
            (best_move, max_eval)
        } else {
            let mut min_eval = f32::INFINITY;
            for m in possible_moves {
                let new_state = board_state.apply_move(&m);
                let (_, eval) = self.alpha_beta(&new_state, depth - 1, alpha, beta, true);
                if eval < min_eval {
                    min_eval = eval;
                    best_move = Some(m.clone());
                }
                beta = beta.min(eval);
                if beta <= alpha {
                    break; // Alpha cutoff
                }
            }
            (best_move, min_eval)
        }
    }

    /// Delegates the evaluation to the wrapped solver.
    fn evaluate(&self, board_state: &BoardState) ->f32 {
        self.scorer.score(board_state)
    }
}

impl Solver for LookAheadSolver {
    fn solve(&self, board_state: &BoardState) -> (Option<BestMove>, f32) {
        let alpha = f32::NEG_INFINITY;
        let beta = f32::INFINITY;

        let (best_move, best_score) = self.alpha_beta(
            board_state,
            self.depth,
            alpha,
            beta,
            true, // Assuming the current player is the maximizing player
        );

        (best_move, best_score)
    }
}
