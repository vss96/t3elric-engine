use std::cmp::{max, min};

use crate::parser::{BestMove, BoardState, Cell, Player};

use super::Solver;

pub struct GreedySolver;

impl Solver for GreedySolver {
    fn solve(board_state: &BoardState) -> Option<BestMove> {
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

                let player_diagonal_score = score_diagonal(
                    &rows.clone(),
                    i as i32,
                    j as i32,
                    &board_state.player_to_move,
                );

                let player_row_score = score_row(&rows, i, j, &board_state.player_to_move);
                let player_column_score = score_column(&rows, i, j, &board_state.player_to_move);
                let opponent_row_score =
                    score_row(&rows, i, j, &board_state.player_to_move.opponent());
                let opponent_column_score =
                    score_column(&rows, i, j, &board_state.player_to_move.opponent());
                let opponent_diagonal_score = score_diagonal(
                    &rows,
                    i as i32,
                    j as i32,
                    &board_state.player_to_move.opponent(),
                );

                let best_player_score = max(
                    player_column_score,
                    max(player_diagonal_score, player_row_score),
                );

                if best_player_score == (rlen as i32 - 1) {
                    return Some(BestMove::new(i as u32, j as u32));
                };

                let row_score = player_row_score - opponent_row_score;
                let column_score = player_column_score - opponent_column_score;
                let diagonal_score = player_diagonal_score - opponent_diagonal_score;

                let player_scores = vec![row_score, column_score, diagonal_score];
                let opponent_scores = vec![-row_score, -column_score, -diagonal_score];
                let losing_score = opponent_scores
                    .clone()
                    .into_iter()
                    .map(|s| 2 * rlen as i32 / max(rlen as i32 - s, 1))
                    .fold(0, |acc, x| acc + x);

                let winning_score = player_scores
                    .into_iter()
                    .map(|s| rlen as i32 / max(rlen as i32 - s, 1))
                    .fold(0, |acc, x| acc + x);

                let scores = vec![
                    row_score,
                    column_score,
                    diagonal_score,
                    winning_score,
                    losing_score,
                ];

                let max_score = scores.clone().into_iter().fold(0, |acc, x| max(acc, x));

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

fn score_column(rows: &Vec<Vec<Cell>>, x: usize, y: usize, player: &Player) -> i32 {
    let r = rows.len();
    let mut score: i32 = 0;
    let player_cell = Cell::Played(player.clone());

    for i in 0..r {
        if rows[i][y] == player_cell {
            score += 1;
        }
    }
    score
}

fn score_row(rows: &Vec<Vec<Cell>>, x: usize, y: usize, player: &Player) -> i32 {
    let c = rows[0].len();
    let mut score: i32 = 0;
    let player_cell = Cell::Played(player.clone());
    for i in 0..c {
        if rows[x][i] == player_cell {
            score += 1;
        }
    }
    score
}

fn score_diagonal(rows: &Vec<Vec<Cell>>, x: i32, y: i32, player: &Player) -> i32 {
    let c = rows[0].len();
    let mut d1score: i32 = 0;
    let mut d2score: i32 = 0;
    let player_cell = Cell::Played(player.clone());

    let d1intercept = (x-y).abs();
    let d2intercept = x + y;

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
        );
        let best_move = GreedySolver::solve(&mut board_state);
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
        );
        let best_move = GreedySolver::solve(&mut board_state);
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
        );
        let best_move = GreedySolver::solve(&mut board_state);
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
        );
        let best_move = GreedySolver::solve(&mut board_state);
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
        );
        let best_move = GreedySolver::solve(&mut board_state);
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
        );
        let best_move = GreedySolver::solve(&mut board_state);
        assert_eq!(best_move, Some(BestMove::new(2, 0)));
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
        );
        let best_move = GreedySolver::solve(&mut board_state);
        assert_eq!(best_move, Some(BestMove::new(2, 1)));
    }
}
