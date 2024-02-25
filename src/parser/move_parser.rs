use std::{borrow::BorrowMut, fmt::Display, iter::repeat, str::FromStr};

use either::Either;

use crate::generate_token_parser;

use super::{
    and_parser::{And2, And3},
    nothing_parser::{Nothing, NothingParser},
    number_parser::Number,
    or_parser::Or3,
    time_parser::{TimeRemainingParser, TimeSetting, TotalTimeParser},
    whitespace_parser::WhiteSpaceParser,
    ParseResult, Parser,
};

#[derive(PartialEq, Eq, Debug)]
pub struct BoardState {
    pub player_to_move: Player,
    pub board: Board,
    pub time_setting: TimeSetting,
}

pub struct BestMove(u32, u32);

impl Display for BestMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut num = self.0;

        let mut chars = Vec::new();
        while num > 0 {
            let remainder = num % 26;
            let ch = (b'a' + remainder as u8) as char;
            chars.push(ch);
            num /= 26;
        }

        // Reverse the characters to get the correct order
        chars.reverse();
        let column = if num == 0 {
            "a".to_string()
        } else {
            chars.iter().collect::<String>()
        };

        write!(f, "best {}{}", column, self.1 + 1)
    }
}

impl BoardState {
    pub fn best_move(&mut self) -> BestMove {
        let (x, y) = self.find_best_move();
        println!("{}, {}", x, y);
        BestMove(x, y)
    }
    // Function to check if the board is full
    fn is_full(&self) -> bool {
        self.board.rows.iter().all(|row| {
            row.iter().all(|cell| match cell {
                Cell::Playable => false,
                _ => true,
            })
        })
    }

    // Function to evaluate the board
    fn evaluate(&self) -> i32 {
        // Evaluation function will depend on the specific requirements
        // This is a simple example that only checks if X wins (+1), O wins (-1), or it's a draw (0)
        // You can expand this to include more complex evaluation criteria
        let mut x_wins = false;
        let mut o_wins = false;

        // Check rows and columns
        for i in 0..3 {
            if self.board.rows[i][0] == self.board.rows[i][1]
                && self.board.rows[i][1] == self.board.rows[i][2]
            {
                match self.board.rows[i][0] {
                    Cell::Played(Player::X) => x_wins = true,
                    Cell::Played(Player::O) => o_wins = true,
                    _ => {}
                }
            }
            if self.board.rows[0][i] == self.board.rows[1][i]
                && self.board.rows[1][i] == self.board.rows[2][i]
            {
                match self.board.rows[0][i] {
                    Cell::Played(Player::X) => x_wins = true,
                    Cell::Played(Player::O) => o_wins = true,
                    _ => {}
                }
            }
        }

        // Check diagonals
        if self.board.rows[0][0] == self.board.rows[1][1]
            && self.board.rows[1][1] == self.board.rows[2][2]
        {
            match self.board.rows[0][0] {
                Cell::Played(Player::X) => x_wins = true,
                Cell::Played(Player::O) => o_wins = true,
                _ => {}
            }
        }
        if self.board.rows[0][2] == self.board.rows[1][1]
            && self.board.rows[1][1] == self.board.rows[2][0]
        {
            match self.board.rows[0][2] {
                Cell::Played(Player::X) => x_wins = true,
                Cell::Played(Player::O) => o_wins = true,
                _ => {}
            }
        }

        if x_wins {
            return 1;
        } else if o_wins {
            return -1;
        } else {
            return 0;
        }
    }

    fn alpha_beta_pruning(
        &mut self,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
        is_maximizing: bool,
    ) -> i32 {
        if depth == 0 || self.is_full() {
            return self.evaluate();
        }

        let mut value;
        if is_maximizing {
            value = std::i32::MIN;
            for i in 0..3 {
                for j in 0..3 {
                    if let Cell::Playable = self.board.rows[i][j] {
                        self.board.rows[i][j] = Cell::Played(self.player_to_move.clone());
                        value = value.max(self.alpha_beta_pruning(depth - 1, alpha, beta, false));
                        self.board.rows[i][j] = Cell::Playable;
                        alpha = alpha.max(value);
                        if alpha >= beta {
                            return value;
                        }
                    }
                }
            }
        } else {
            value = std::i32::MAX;
            for i in 0..3 {
                for j in 0..3 {
                    if let Cell::Playable = self.board.rows[i][j] {
                        let opponent = match self.player_to_move {
                            Player::X => Player::O,
                            Player::O => Player::X,
                        };
                        self.board.rows[i][j] = Cell::Played(opponent);
                        value = value.min(self.alpha_beta_pruning(depth - 1, alpha, beta, true));
                        self.board.rows[i][j] = Cell::Playable;
                        beta = beta.min(value);
                        if alpha >= beta {
                            return value;
                        }
                    }
                }
            }
        }
        value
    }

    fn find_best_move(&mut self) -> (u32, u32) {
        let mut best_move: (u32, u32) = (0, 0);
        let mut best_value = std::i32::MIN;

        for i in 0..3 {
            for j in 0..3 {
                if let Cell::Playable = &self.board.rows[i][j] {
                    self.board.rows[i][j] = Cell::Played(self.player_to_move.clone());
                    let value = self.alpha_beta_pruning(9, std::i32::MIN, std::i32::MAX, false);
                    self.board.rows[i][j] = Cell::Playable;
                    if value > best_value {
                        best_value = value;
                        best_move = (i as u32, j as u32);
                    }
                }
            }
        }
        best_move
    }
}

pub struct MoveTokenParser;
pub const MOVE: &str = "move";

generate_token_parser!(MOVE, MoveTokenParser);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Cell {
    Playable,
    NonPlayable,
    Played(Player),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Player {
    X,
    O,
}

impl FromStr for Player {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Player::X),
            "o" => Ok(Player::O),
            _ => Err(format!("{} is not a valid player.", s)),
        }
    }
}

pub struct PlayerParser;

impl Parser<Player> for PlayerParser {
    fn parse_from(val: &String) -> ParseResult<Player> {
        if val.is_empty() {
            return Err("Expected a non empty string".to_string());
        }
        Ok((Player::from_str(&val[..1])?, val[1..].to_string()))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    pub rows: Vec<Vec<Cell>>,
}

impl FromStr for Cell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "_" => Ok(Cell::Playable),
            "." => Ok(Cell::NonPlayable),
            "x" => Ok(Cell::Played(Player::X)),
            "o" => Ok(Cell::Played(Player::O)),
            _ => Err(format!("Invalid cell representation: {}", s)),
        }
    }
}

impl Cell {
    fn from_row(row: &str) -> Result<Vec<Cell>, String> {
        let mut board_row: Vec<Cell> = vec![];

        let mut iter = row.chars().peekable();

        while let Some(c) = iter.next() {
            match c {
                '1'..='9' => {
                    let mut numric_string: String = c.to_string();
                    while let Some(&next_char) = iter.peek() {
                        if next_char.is_numeric() {
                            numric_string.push(next_char);
                            iter.next();
                        } else {
                            break;
                        }
                    }
                    let number = Number::try_from(&numric_string)?;

                    if let Some(x) = iter.peek() {
                        board_row.extend(
                            repeat(Cell::from_str(&x.to_string())?).take((number.0) as usize),
                        );
                        iter.next();
                    } else {
                        return Err(String::from("Invalid row, row can't end with a number"));
                    }
                }
                _ => {
                    let cell = Cell::from_str(&c.to_string())?;
                    board_row.push(cell);
                }
            }
        }

        Ok(board_row)
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<Cell>> = s
            .split('/')
            .map(|row| Cell::from_row(row))
            .collect::<Result<Vec<Vec<Cell>>, String>>()?;

        Ok(Board { rows })
    }
}

pub struct BoardParser;

impl Parser<Board> for BoardParser {
    fn parse_from(val: &String) -> ParseResult<Board> {
        if let Some((board_string, rest)) = val.split_once(' ') {
            return Ok((Board::from_str(board_string)?, rest.to_string()));
        }
        Err("Invalid T3N notation".to_string())
    }
}
pub type T3NParser = And2<BoardParser, PlayerParser>;

pub type BasicMoveParser = And3<MoveTokenParser, WhiteSpaceParser, T3NParser>;

pub type MoveWithInfiniteTimeParser = And2<BasicMoveParser, NothingParser>;
pub type MoveWithTotalTime = And3<BasicMoveParser, WhiteSpaceParser, TotalTimeParser>;
pub type MoveWithTimeRemaining = And3<BasicMoveParser, WhiteSpaceParser, TimeRemainingParser>;

pub type MoveParser = Or3<MoveWithTotalTime, MoveWithTimeRemaining, MoveWithInfiniteTimeParser>;
pub type MoveParserReturnType = Either<
    (
        (String, (String, (Board, Player))),
        (String, (String, (String, Number))),
    ),
    Either<
        (
            (String, (String, (Board, Player))),
            (String, (String, (String, Number))),
        ),
        ((String, (String, (Board, Player))), Nothing),
    >,
>;

pub fn map_to_move(parser_output: MoveParserReturnType) -> BoardState {
    match parser_output {
        Either::Left(((_, (_, (board, player))), (_, (_, (_, time))))) => BoardState {
            player_to_move: player,
            board,
            time_setting: TimeSetting::TotalTime(time),
        },
        Either::Right(Either::Left(((_, (_, (board, player))), (_, (_, (_, time)))))) => {
            BoardState {
                player_to_move: player,
                board,
                time_setting: TimeSetting::TimeRemaining(time),
            }
        }

        Either::Right(Either::Right(((_, (_, (board, player))), _))) => BoardState {
            player_to_move: player,
            board,
            time_setting: TimeSetting::Infinite,
        },
    }
}

#[cfg(test)]
mod test_t3nparser {

    use crate::parser::{
        move_parser::{
            Board, BoardParser,
            Cell::{NonPlayable, Playable, Played},
            Player,
        },
        Parser,
    };

    use super::Cell;

    #[test]
    fn parse_t3n_board() {
        let board_string = String::from("3_.x/4_o/5. x time-remaining ms:1500ms");
        let board = BoardParser::parse_from(&board_string);

        assert_eq!(
            Ok((
                Board {
                    rows: vec![
                        vec![Playable, Playable, Playable, NonPlayable, Played(Player::X)],
                        vec![Playable, Playable, Playable, Playable, Played(Player::O)],
                        vec![
                            NonPlayable,
                            NonPlayable,
                            NonPlayable,
                            NonPlayable,
                            NonPlayable
                        ]
                    ]
                },
                String::from("x time-remaining ms:1500ms")
            )),
            board
        )
    }

    #[test]
    fn parse_t3n_row() {
        let row_string = "5_";
        let board_row = Cell::from_row(&row_string);
        assert_eq!(
            Ok(vec![Playable, Playable, Playable, Playable, Playable]),
            board_row
        );
    }
}

#[cfg(test)]
mod test_move_parser {
    use either::Either::{Left, Right};

    use crate::parser::{nothing_parser::Nothing, Parser};

    use super::*;

    use Cell::{Playable, Played};

    #[test]
    fn parse_move_without_time() {
        let move_string = String::from("move 3_/_x_/3_ o");

        let res = MoveParser::parse_from(&move_string);

        assert_eq!(
            res,
            Ok((
                Right(Right((
                    ((
                        String::from("move"),
                        (
                            String::from(" "),
                            (
                                Board {
                                    rows: vec![
                                        vec![Playable, Playable, Playable],
                                        vec![Playable, Played(Player::X), Playable],
                                        vec![Playable, Playable, Playable]
                                    ]
                                },
                                Player::O
                            )
                        )
                    )),
                    Nothing
                ))),
                String::from("")
            ))
        );
    }

    #[test]
    fn parse_move_with_total_time() {
        let move_string = String::from("move 3_/3_/3_ x time ms:1000");

        let res = MoveParser::parse_from(&move_string);

        assert_eq!(
            res,
            Ok((
                Left((
                    ((
                        String::from("move"),
                        (
                            " ".to_string(),
                            (
                                Board {
                                    rows: vec![
                                        vec![Playable, Playable, Playable],
                                        vec![Playable, Playable, Playable],
                                        vec![Playable, Playable, Playable]
                                    ]
                                },
                                Player::X
                            )
                        )
                    )),
                    (
                        " ".to_string(),
                        ("time".to_string(), (" ".to_string(), Number(1000)))
                    )
                )),
                "".to_string()
            ))
        );
    }

    #[test]
    fn parse_move_with_remaining_time() {
        let move_string = String::from("move 3_/3_/3_ x time-remaining ms:10000");

        let res = MoveParser::parse_from(&move_string);

        assert_eq!(
            res,
            Ok((
                Right(Left((
                    ((
                        String::from("move"),
                        (
                            " ".to_string(),
                            (
                                Board {
                                    rows: vec![
                                        vec![Playable, Playable, Playable],
                                        vec![Playable, Playable, Playable],
                                        vec![Playable, Playable, Playable]
                                    ]
                                },
                                Player::X
                            )
                        )
                    )),
                    (
                        " ".to_string(),
                        (
                            "time-remaining".to_string(),
                            (" ".to_string(), Number(10000))
                        )
                    )
                ))),
                "".to_string()
            ))
        );
    }
}
