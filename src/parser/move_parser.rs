use std::{fmt::Display, iter::repeat, str::FromStr};

use super::{
    and_parser::{AndParser, TriAnd},
    nothing_parser::NothingParser,
    number_parser::Number,
    or_parser::TriOr,
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

impl BestMove {
    pub fn format_move(&self) -> String {
        let mut num = self.0;
        if num == 0 {
            return "a".to_string();
        }

        let mut chars = Vec::new();
        while num > 0 {
            let remainder = num % 26;
            let ch = (b'a' + remainder as u8) as char;
            chars.push(ch);
            num /= 26;
        }

        // Reverse the characters to get the correct order
        chars.reverse();

        format!("{}{}", chars.iter().collect::<String>(), self.1)
    }
}

impl Display for BestMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "best {}", self.format_move())
    }
}

impl BoardState {
    pub fn best_move(&self) -> BestMove {
        BestMove(0, 0)
    }
}

pub struct MoveTokenParser;
pub const MOVE: &str = "move";

impl Parser<String> for MoveTokenParser {
    fn parse_from(val: &String) -> ParseResult<String> {
        if val.starts_with(MOVE) {
            return Ok((MOVE.to_string(), val[MOVE.len()..].to_string()));
        }

        Err("Invalid move type".to_string())
    }
}

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
    rows: Vec<Vec<Cell>>,
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

pub struct T3nParser;

impl Parser<Board> for T3nParser {
    fn parse_from(val: &String) -> ParseResult<Board> {
        if let Some((board_string, rest)) = val.split_once(' ') {
            return Ok((Board::from_str(board_string)?, rest.to_string()));
        }
        Err("Invalid T3N notation".to_string())
    }
}

pub type BasicMoveParser =
    AndParser<TriAnd<MoveTokenParser, WhiteSpaceParser, T3nParser>, PlayerParser>;

pub type MoveWithInfiniteTimeParser = AndParser<BasicMoveParser, NothingParser>;
pub type MoveWithTotalTime = TriAnd<BasicMoveParser, WhiteSpaceParser, TotalTimeParser>;
pub type MoveWithTimeRemaining = TriAnd<BasicMoveParser, WhiteSpaceParser, TimeRemainingParser>;

pub type MoveParser = TriOr<MoveWithTotalTime, MoveWithTimeRemaining, MoveWithInfiniteTimeParser>;

#[cfg(test)]
mod test_t3nparser {

    use crate::parser::{
        move_parser::{
            Board,
            Cell::{NonPlayable, Playable, Played},
            Player, T3nParser,
        },
        Parser,
    };

    use super::Cell;

    #[test]
    fn parse_t3n_board() {
        let board_string = String::from("3_.x/4_o/5. x time-remaining ms:1500ms");
        let board = T3nParser::parse_from(&board_string);

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
                    (
                        (
                            String::from("move"),
                            (
                                String::from(" "),
                                Board {
                                    rows: vec![
                                        vec![Playable, Playable, Playable],
                                        vec![Playable, Played(Player::X), Playable],
                                        vec![Playable, Playable, Playable]
                                    ]
                                }
                            )
                        ),
                        Player::O
                    ),
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
                    (
                        (
                            String::from("move"),
                            (
                                " ".to_string(),
                                Board {
                                    rows: vec![
                                        vec![Playable, Playable, Playable],
                                        vec![Playable, Playable, Playable],
                                        vec![Playable, Playable, Playable]
                                    ]
                                }
                            )
                        ),
                        Player::X
                    ),
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
                    (
                        (
                            String::from("move"),
                            (
                                " ".to_string(),
                                Board {
                                    rows: vec![
                                        vec![Playable, Playable, Playable],
                                        vec![Playable, Playable, Playable],
                                        vec![Playable, Playable, Playable]
                                    ]
                                }
                            )
                        ),
                        Player::X
                    ),
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
