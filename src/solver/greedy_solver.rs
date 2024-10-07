use std::cmp::max;

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
}

impl Default for GreedySolver {
    fn default() -> Self {
        let diff_evaluators: Vec<Box<dyn Evaluator>> = vec![
            Box::new(WinningEvaluator::new(Box::new(ReduceEvaluator::new(
                vec![
                    Box::new(MapEvaluator::new(
                        Box::new(OpponentEvaluator::new(Box::new(ColumnEvaluator))),
                        Box::new(|f| -1 * f),
                    )),
                    Box::new(ColumnEvaluator),
                ],
                Box::new(|x, y| x + y),
            )))),
            Box::new(WinningEvaluator::new(Box::new(ReduceEvaluator::new(
                vec![
                    Box::new(MapEvaluator::new(
                        Box::new(OpponentEvaluator::new(Box::new(RowEvaluator))),
                        Box::new(|f| -1 * f),
                    )),
                    Box::new(RowEvaluator),
                ],
                Box::new(|x, y| x + y),
            )))),
            Box::new(WinningEvaluator::new(Box::new(ReduceEvaluator::new(
                vec![
                    Box::new(MapEvaluator::new(
                        Box::new(OpponentEvaluator::new(Box::new(DiagonalEvaluator))),
                        Box::new(|f| -1 * f),
                    )),
                    Box::new(DiagonalEvaluator),
                ],
                Box::new(|x, y| x + y),
            )))),
        ];

        let losing_diff_evaluators: Vec<Box<dyn Evaluator>> = vec![
            Box::new(WinningEvaluator::new(Box::new(ReduceEvaluator::new(
                vec![
                    Box::new(MapEvaluator::new(
                        Box::new(ColumnEvaluator),
                        Box::new(|f| -1 * f),
                    )),
                    Box::new(OpponentEvaluator::new(Box::new(ColumnEvaluator))),
                ],
                Box::new(|x, y| x + y),
            )))),
            Box::new(WinningEvaluator::new(Box::new(ReduceEvaluator::new(
                vec![
                    Box::new(MapEvaluator::new(
                        Box::new(RowEvaluator),
                        Box::new(|f| -1 * f),
                    )),
                    Box::new(OpponentEvaluator::new(Box::new(RowEvaluator))),
                ],
                Box::new(|x, y| x + y),
            )))),
            Box::new(WinningEvaluator::new(Box::new(ReduceEvaluator::new(
                vec![
                    Box::new(MapEvaluator::new(
                        Box::new(DiagonalEvaluator),
                        Box::new(|f| -1 * f),
                    )),
                    Box::new(OpponentEvaluator::new(Box::new(DiagonalEvaluator))),
                ],
                Box::new(|x, y| x + y),
            )))),
        ];

        let winning_evaluator: Box<dyn Evaluator> = Box::new(ReduceEvaluator::new(
            diff_evaluators,
            Box::new(|x, y| x + y),
        ));
        let losing_evaluator: Box<dyn Evaluator> = Box::new(MapEvaluator::new(
            Box::new(ReduceEvaluator::new(
                losing_diff_evaluators,
                Box::new(|x, y| x + y),
            )),
            Box::new(|f| f * 2),
        ));

        let evaluators: Vec<Box<dyn Evaluator>> = vec![
            Box::new(ColumnEvaluator),
            Box::new(RowEvaluator),
            Box::new(DiagonalEvaluator),
        ];
        let player_score_evaluator =
            Box::new(ReduceEvaluator::new(evaluators, Box::new(|x, y| max(x, y))));
        Self {
            winning_evaluator,
            losing_evaluator,
            player_score_evaluator,
        }
    }
}

impl Solver for GreedySolver {
    fn solve(&self, board_state: &BoardState) -> Option<BestMove> {
        let rows = &board_state.board.get_rows();
        let (rlen, clen) = (rows.len(), rows[0].len());

        let no_of_moves = rows
            .into_iter()
            .flatten()
            .filter(|c| **c == Cell::Played(board_state.player_to_move.clone()))
            .count();

        if rows[rlen / 2][clen / 2] == Cell::Playable && no_of_moves < 1 {
            return Some(BestMove::new(rlen as u32 / 2, clen as u32 / 2));
        }
        let mut best_score = -1;
        let (mut bx, mut by) = (-1, -1);
        for i in 0..rlen {
            for j in 0..clen {
                let cell: &Cell = &rows[i][j];
                if *cell != Cell::Playable {
                    continue;
                }

                let scores = vec![
                    self.player_score_evaluator
                        .score(board_state, i, j, &board_state.player_to_move),
                    self.winning_evaluator
                        .score(board_state, i, j, &board_state.player_to_move),
                    self.losing_evaluator
                        .score(board_state, i, j, &board_state.player_to_move),
                ];

                if scores[0] == (board_state.win_length as i32 - 1) {
                    return Some(BestMove::new(i as u32, j as u32));
                };

                let max_score = scores.iter().fold(0, |acc, x| max(acc, *x));

                if max_score > best_score {
                    best_score = max_score;
                    (bx, by) = (i as i32, j as i32);
                }
            }
        }

        if (bx, by) == (-1, -1) {
            None
        } else {
            Some(BestMove::new(bx as u32, by as u32))
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
            None
        );
        let best_move = GreedySolver::default().solve(&mut board_state);
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
            None
        );
        let best_move = GreedySolver::default().solve(&mut board_state);
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
            None
        );
        let best_move = GreedySolver::default().solve(&mut board_state);
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
            None
        );
        let best_move = GreedySolver::default().solve(&mut board_state);
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
            None
        );
        let best_move = GreedySolver::default().solve(&mut board_state);
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
            None
        );
        let best_move = GreedySolver::default().solve(&mut board_state);
        assert_eq!(best_move, Some(BestMove::new(2, 2)));
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
            None
        );
        let best_move = GreedySolver::default().solve(&mut board_state);
        assert_eq!(best_move, Some(BestMove::new(2, 1)));
    }
}
