use std::cmp::max;

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
                let x = &rows[i][j];
                if *x != Cell::Playable {
                    continue;
                }

                let diagonal_score = check_diagonal(
                    &rows.clone(),
                    i as i32,
                    j as i32,
                    &board_state.player_to_move,
                );

                let player_row_score = check_row(&rows, i, j, &board_state.player_to_move);
                let player_column_score = check_column(&rows, i, j, &board_state.player_to_move);
                let opponent_row_score =
                    check_row(&rows, i, j, &board_state.player_to_move.opponent());
                let opponent_column_score =
                    check_column(&rows, i, j, &board_state.player_to_move.opponent());
                let block_score = check_block(&rows, i, j, &board_state.player_to_move);

                let row_score = player_row_score - opponent_row_score;
                let column_score = player_column_score - opponent_column_score;

                let winning_row_score = rlen / max(rlen - player_row_score as usize, 1);
                let winning_col_score = clen / max(clen - player_column_score as usize, 1);
                let winning_dia_score = clen / max(rlen - diagonal_score as usize, 1);

                let scores = vec![
                    row_score,
                    column_score,
                    diagonal_score,
                    block_score,
                    winning_row_score as i32,
                    winning_col_score as i32,
                    winning_dia_score as i32,
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

fn check_block(rows: &Vec<Vec<Cell>>, x: usize, y: usize, player: &Player) -> i32 {
    let opponent = player.opponent();
    return max(
        check_column(rows, x, y, &opponent),
        max(
            check_row(rows, x, y, &opponent),
            check_diagonal(rows, x as i32, y as i32, &opponent),
        ),
    );
}

fn check_column(rows: &Vec<Vec<Cell>>, x: usize, y: usize, player: &Player) -> i32 {
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

fn check_row(rows: &Vec<Vec<Cell>>, x: usize, y: usize, player: &Player) -> i32 {
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

fn check_diagonal(rows: &Vec<Vec<Cell>>, x: i32, y: i32, player: &Player) -> i32 {
    let c = rows[0].len();
    let mut d1score: i32 = 0;
    let mut d2score: i32 = 0;
    let player_cell = Cell::Played(player.clone());

    let (d1x1, d1y1) = (0, 0);
    let (d2x1, d2y1) = (x, 0);

    let (d1xdiff, d1ydiff) = (d1x1 - x, d1y1 - y);
    let (d2xdiff, d2ydiff) = (d2x1 - x, d2y1 - y);

    if d1xdiff.abs() == d1ydiff.abs() {
        for i in 0..c {
            if rows[i][i] == player_cell {
                d1score += 1;
            }
        }
    }

    if d2xdiff.abs() == d2ydiff.abs() {
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
    fn should_pick_winning_move_over_block() {
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
    fn should_block_opponent_winning_move() {
        let mut board_state = BoardState::new(
            Player::X,
            Board::new(vec![
                vec![Playable, Playable, Playable],
                vec![Playable, Played(Player::X), Played(Player::O)],
                vec![Playable, Playable, Played(Player::O)],
            ]),
            TimeSetting::Infinite,
        );
        let best_move = GreedySolver::solve(&mut board_state);
        assert_eq!(best_move, Some(BestMove::new(0, 2)));
    }

    #[test]
    fn should_use_move() {
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
}
