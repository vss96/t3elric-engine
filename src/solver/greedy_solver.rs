
use crate::{
    evaluator::{
        ColumnEvaluator, DiagonalEvaluator, Evaluator, MapEvaluator, OpponentEvaluator,
        ReduceEvaluator, RowEvaluator, WinningEvaluator,
    },
    parser::{BestMove, BoardState, Cell},
};

use super::Solver;

pub struct GreedySolver {
    winning_evaluator: Box<dyn Evaluator>,
    losing_evaluator: Box<dyn Evaluator>,
    player_score_evaluator: Box<dyn Evaluator>,
    player_sum_evaluator: Box<dyn Evaluator>,
}

impl Default for GreedySolver {
    fn default() -> Self {
        let win_evaluators: Vec<Box<dyn Evaluator>> = vec![
            Box::new(WinningEvaluator::new(Box::new(ColumnEvaluator))),
            Box::new(WinningEvaluator::new(Box::new(RowEvaluator))),
            Box::new(WinningEvaluator::new(Box::new(DiagonalEvaluator))),
        ];

        let losing_evaluators: Vec<Box<dyn Evaluator>> = vec![
            Box::new(WinningEvaluator::new(Box::new(OpponentEvaluator::new(
                Box::new(ColumnEvaluator),
            )))),
            Box::new(WinningEvaluator::new(Box::new(OpponentEvaluator::new(
                Box::new(RowEvaluator),
            )))),
            Box::new(WinningEvaluator::new(Box::new(OpponentEvaluator::new(
                Box::new(DiagonalEvaluator),
            )))),
        ];

        let winning_evaluator: Box<dyn Evaluator> = Box::new(ReduceEvaluator::new(
            win_evaluators,
            Box::new(|x, y| f32::max(x, y)),
        ));
        let losing_evaluator: Box<dyn Evaluator> = Box::new(MapEvaluator::new(
            Box::new(ReduceEvaluator::new(
                losing_evaluators,
                Box::new(|x, y| f32::max(x, y)),
            )),
            Box::new(|f| f * 1.0),
        ));

        let player_score_evaluator = Box::new(ReduceEvaluator::new(
            vec![
                Box::new(ColumnEvaluator),
                Box::new(RowEvaluator),
                Box::new(DiagonalEvaluator),
            ],
            Box::new(|x, y| f32::max(x, y)),
        ));

        let player_sum_evaluator = Box::new(ReduceEvaluator::new(
            vec![
                Box::new(ColumnEvaluator),
                Box::new(RowEvaluator),
                Box::new(DiagonalEvaluator),
            ],
            Box::new(|x, y| x + y),
        ));

        Self {
            winning_evaluator,
            losing_evaluator,
            player_score_evaluator,
            player_sum_evaluator,
        }
    }
}

impl Solver for GreedySolver {
    fn solve(&self, board_state: &BoardState) -> (Option<BestMove>, f32) {
        let rows = &board_state.board.get_rows();
        let (rlen, clen) = (rows.len(), rows[0].len());

        // let no_of_moves = rows
        //     .into_iter()
        //     .flatten()
        //     .filter(|c| **c == Cell::Played(board_state.player_to_move.clone()))
        //     .count();

        // if rows[rlen / 2][clen / 2] == Cell::Playable && no_of_moves < 1 {
        //     return (Some(BestMove::new(rlen as u32 / 2, clen as u32 / 2)), 100.0);
        // }
        let mut best_score = -1f32;
        let (mut bx, mut by) = (-1, -1);
        for i in 0..rlen {
            for j in 0..clen {
                let cell: &Cell = &rows[i][j];
                if *cell != Cell::Playable {
                    continue;
                }

                let scores = vec![
                    self.player_score_evaluator.score(
                        board_state,
                        i,
                        j,
                        &board_state.player_to_move,
                    ),
                    self.winning_evaluator
                        .score(board_state, i, j, &board_state.player_to_move),
                    self.losing_evaluator
                        .score(board_state, i, j, &board_state.player_to_move),
                    self.player_sum_evaluator
                        .score(board_state, i, j, &board_state.player_to_move),
                ];

                // println!("({}, {}) -> {:?}", i, j, scores);

                if scores[0] == (board_state.win_length as f32 - 1f32) {
                    return (Some(BestMove::new(i as u32, j as u32)), 1000.0);
                };

                let max_score = scores.iter().fold(0.0, |acc, x| f32::max(acc, *x));

                if max_score > best_score {
                    best_score = max_score;
                    (bx, by) = (i as i32, j as i32);
                }
            }
        }

        if (bx, by) == (-1, -1) {
            (None, -1.0)
        } else {
            (Some(BestMove::new(bx as u32, by as u32)), best_score)
        }
    }
}

#[cfg(test)]
mod test_greedy_solver {
    use crate::{
        parser::{BestMove, Board, BoardState, Cell::Playable, Cell::Played, Player, TimeSetting},
        solver::Solver,
    };

    use super::GreedySolver;

    #[test]
    fn should_find_winning_move1() {
        let mut board_state = BoardState::new(
            Player::X,
            Board::new(vec![
                vec![Played(Player::O), Playable, Playable],
                vec![Playable, Played(Player::O), Played(Player::O)],
                vec![Playable, Played(Player::X), Played(Player::X)],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let best_move = GreedySolver::default().solve(&mut board_state).0;
        assert_eq!(best_move, Some(BestMove::new(2, 0)));
    }

    #[test]
    fn should_block_winning_move1() {
        let mut board_state = BoardState::new(
            Player::X,
            Board::new(vec![
                vec![Playable, Playable, Playable],
                vec![Played(Player::X), Played(Player::X), Played(Player::O)],
                vec![Playable, Playable, Played(Player::O)],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let best_move = GreedySolver::default().solve(&mut board_state).0;
        assert_eq!(best_move, Some(BestMove::new(0, 2)));
    }

    #[test]
    fn should_find_winning_move2() {
        let mut board_state = BoardState::new(
            Player::O,
            Board::new(vec![
                vec![Played(Player::O), Played(Player::X), Played(Player::O)],
                vec![Playable, Played(Player::O), Played(Player::X)],
                vec![Playable, Played(Player::X), Played(Player::X)],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let best_move = GreedySolver::default().solve(&mut board_state).0;
        assert_eq!(best_move, Some(BestMove::new(2, 0)));
    }

    #[test]
    fn should_find_winning_move3() {
        let mut board_state = BoardState::new(
            Player::X,
            Board::new(vec![
                vec![Played(Player::O), Playable, Playable],
                vec![Playable, Played(Player::X), Played(Player::O)],
                vec![Playable, Played(Player::X), Played(Player::O)],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let best_move = GreedySolver::default().solve(&mut board_state).0;
        assert_eq!(best_move, Some(BestMove::new(0, 1)));
    }

    #[test]
    fn should_block_winning_move2() {
        let mut board_state = BoardState::new(
            Player::O,
            Board::new(vec![
                vec![Played(Player::O), Playable, Playable],
                vec![Played(Player::X), Played(Player::X), Played(Player::O)],
                vec![Played(Player::X), Played(Player::O), Played(Player::X)],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let best_move = GreedySolver::default().solve(&mut board_state).0;
        assert_eq!(best_move, Some(BestMove::new(0, 2)));
    }

    #[test]
    fn should_find_winning_move4() {
        let mut board_state = BoardState::new(
            Player::X,
            Board::new(vec![
                vec![Played(Player::O), Played(Player::X), Playable],
                vec![Playable, Played(Player::X), Playable],
                vec![Playable, Played(Player::O), Playable],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let best_move = GreedySolver::default().solve(&mut board_state).0;
        assert_eq!(best_move, Some(BestMove::new(1, 0)));
    }

    #[test]
    fn should_block_winning_move3() {
        let mut board_state = BoardState::new(
            Player::O,
            Board::new(vec![
                vec![Played(Player::O), Played(Player::X), Playable],
                vec![Playable, Played(Player::X), Playable],
                vec![Playable, Playable, Playable],
            ]),
            TimeSetting::Infinite,
            None,
        );
        let best_move = GreedySolver::default().solve(&mut board_state).0;
        assert_eq!(best_move, Some(BestMove::new(2, 1)));
    }

    #[test]
    fn should_find_winning_move5() {
        let mut board_state = BoardState::new(
            Player::O,
            Board::new(vec![
                vec![Playable, Playable, Playable, Playable, Played(Player::O)],
                vec![Playable, Playable, Playable, Played(Player::X), Playable],
                vec![Playable, Playable, Played(Player::O), Playable, Playable],
                vec![Playable, Played(Player::X), Playable, Playable, Playable],
                vec![Played(Player::X), Playable, Playable, Playable, Playable],
            ]),
            TimeSetting::Infinite,
            Some(3),
        );
        let best_move = GreedySolver::default().solve(&mut board_state).0;
        assert_eq!(best_move, Some(BestMove::new(0, 2)));
    }

    #[test]
    fn should_find_winning_move6() {
        let mut board_state = BoardState::new(
            Player::X,
            Board::new(vec![
                vec![Playable, Playable, Playable, Playable, Playable],
                vec![Playable, Playable, Playable, Playable, Playable],
                vec![Playable, Playable, Played(Player::O), Playable, Playable],
                vec![Playable, Played(Player::X), Playable, Playable, Playable],
                vec![Played(Player::X), Playable, Playable, Playable, Playable],
            ]),
            TimeSetting::Infinite,
            Some(3),
        );
        let best_move = GreedySolver::default().solve(&mut board_state).0;
        assert_eq!(best_move, Some(BestMove::new(3, 0)));
    }
}
