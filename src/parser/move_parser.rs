use std::{fmt::Display, iter::repeat, str::FromStr};

use either::Either;

use crate::generate_token_parser;

use super::{
    and_parser::{And2, And3},
    anything_parser::{Anything, AnythingParser},
    nothing_parser::{Nothing, NothingParser},
    number_parser::Number,
    or_parser::{Or2, Or3},
    time_parser::{TimeRemainingParser, TimeSetting, TotalTimeParser},
    whitespace_parser::WhiteSpaceParser,
    ParseResult, Parser,
};


#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BestMove(u32, u32);

impl BestMove {
    pub fn new(x: u32, y: u32) -> Self {
        Self(x, y)
    }

    pub fn get_column_notation(&self) -> String {
        let mut num = self.1;
        let mut chars = Vec::new();
        if num == 0 {
            return "a".to_string();
        }
        while num > 0 {
            let remainder = num % 26;
            let ch = (b'a' + remainder as u8) as char;
            chars.push(ch);
            num /= 26;
        }

        // Reverse the characters to get the correct order
        chars.reverse();
        chars.iter().collect::<String>()
    }

    pub fn get_row_notation(&self) -> u32 {
        self.0 + 1
    }
}

impl Display for BestMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "best {}{}",
            self.get_column_notation(),
            self.get_row_notation()
        )
    }
}



#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BoardState {
    pub player_to_move: Player,
    pub board: Board,
    pub time_setting: TimeSetting,
    pub win_length: u32,
    pub winner: Option<Player>,
}

impl BoardState {
    pub fn new(
        player_to_move: Player,
        board: Board,
        time_setting: TimeSetting,
        win_length: Option<u32>,
    ) -> Self {
        let default_win_length: u32 = board.clone().rows.len() as u32;
        Self {
            player_to_move,
            board,
            time_setting,
            win_length: win_length.unwrap_or(default_win_length),
            winner: None
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.board
            .check_win(self.player_to_move.opponent(), self.win_length)
            || self.board.check_win(self.player_to_move, self.win_length)
            || self.get_possible_moves().is_empty()
    }

    pub fn get_winner(&mut self) -> Option<Player> {

        if self.winner.is_some() {
            return self.winner
        }

        if self.board.check_win(self.player_to_move.opponent(), self.win_length) {
            self.winner = Some(self.player_to_move.opponent());
            return self.winner;
        } 
        
        if self.board.check_win(self.player_to_move, self.win_length) {
             self.winner = Some(self.player_to_move);
             return self.winner;
        }
        None
    }

    pub fn get_possible_moves(&self) -> Vec<BestMove> {
        let mut moves = Vec::new();
        let rows = self.board.get_rows();
        for i in 0..rows.len() {
            for j in 0..rows[0].len() {
                if rows[i][j] == Cell::Playable {
                    moves.push(BestMove::new(i as u32, j as u32));
                }
            }
        }
        moves
    }

    pub fn apply_move(&self, mv: &BestMove) -> BoardState {
        let mut new_board = self.board.clone();
        new_board.play_move(mv.0, mv.1, self.player_to_move.clone());
        BoardState {
            player_to_move: self.player_to_move.opponent(),
            board: new_board,
            time_setting: self.time_setting.clone(),
            win_length: self.win_length,
            winner: None
        }
    }
}

#[cfg(test)]
mod board_state_tests {
    use crate::parser::{Board, BoardState, Cell, Player, TimeSetting};


    #[test]
    fn test_is_game_over() {
        // Test case 1: Game won by Player X with a horizontal win
        let board = Board {
            rows: vec![
                vec![Cell::Played(Player::X), Cell::Played(Player::X), Cell::Played(Player::X)],
                vec![Cell::Playable, Cell::Playable, Cell::Playable],
                vec![Cell::Playable, Cell::Playable, Cell::Playable],
            ],
        };

        let board_state = BoardState::new(Player::X, board, TimeSetting::Infinite, Some(3));
        assert!(board_state.is_game_over(), "Game should be over as Player X has won horizontally.");
    }

    #[test]
    fn test_draw_no_winner() {
        // Board full with no winner (Draw)
        let board = Board {
            rows: vec![
                vec![Cell::Played(Player::X), Cell::Played(Player::O), Cell::Played(Player::X)],
                vec![Cell::Played(Player::O), Cell::Played(Player::X), Cell::Played(Player::O)],
                vec![Cell::Played(Player::O), Cell::Played(Player::X), Cell::Played(Player::O)],
            ],
        };

        let board_state = BoardState::new(Player::X, board, TimeSetting::Infinite, Some(3));
        assert!(board_state.is_game_over(), "Game should be over due to a full board with no winner.");
    }

    #[test]
    fn test_game_ongoing_with_moves_remaining() {
        // Game ongoing with playable moves and no winner
        let board = Board {
            rows: vec![
                vec![Cell::Playable, Cell::Playable, Cell::Playable],
                vec![Cell::Playable, Cell::Played(Player::X), Cell::Playable],
                vec![Cell::Playable, Cell::Playable, Cell::Playable],
            ],
        };

        let board_state = BoardState::new(Player::X, board, TimeSetting::Infinite, Some(3));
        assert!(!board_state.is_game_over(), "Game should not be over as there are possible moves and no winner.");
    }

    #[test]
    fn test_get_winner_player_x_wins() {
        // Player X wins with a horizontal line
        let board = Board {
            rows: vec![
                vec![Cell::Played(Player::X), Cell::Played(Player::X), Cell::Played(Player::X)],
                vec![Cell::Playable, Cell::Playable, Cell::Playable],
                vec![Cell::Playable, Cell::Playable, Cell::Playable],
            ],
        };

        let mut board_state = BoardState::new(Player::X, board, TimeSetting::Infinite, Some(3));
        assert_eq!(board_state.get_winner(), Some(Player::X), "Player X should be the winner.");
    }


    #[test]
    fn test_get_diagonal_winner_player_x_wins() {
        // Player X wins with a horizontal line
        let board = Board {
            rows: vec![
                vec![Cell::Played(Player::X), Cell::Played(Player::O), Cell::Played(Player::O)],
                vec![Cell::Playable, Cell::Played(Player::X), Cell::Playable],
                vec![Cell::Playable, Cell::Playable, Cell::Played(Player::X)],
            ],
        };

        let mut board_state = BoardState::new(Player::X, board, TimeSetting::Infinite, Some(3));
        assert_eq!(board_state.get_winner(), Some(Player::X), "Player X should be the winner.");
    }

    #[test]
    fn test_get_winner_player_o_wins() {
        // Player O wins with a vertical line
        let board = Board {
            rows: vec![
                vec![Cell::Played(Player::O), Cell::Playable, Cell::Playable],
                vec![Cell::Played(Player::O), Cell::Playable, Cell::Playable],
                vec![Cell::Played(Player::O), Cell::Playable, Cell::Playable],
            ],
        };

        let mut board_state = BoardState::new(Player::O, board, TimeSetting::Infinite, Some(3));
        assert_eq!(board_state.get_winner(), Some(Player::O), "Player O should be the winner.");
    }

    #[test]
    fn test_get_winner_draw_no_winner() {
        // Full board with no winner (draw)
        let board = Board {
            rows: vec![
                vec![Cell::Played(Player::X), Cell::Played(Player::O), Cell::Played(Player::X)],
                vec![Cell::Played(Player::O), Cell::Played(Player::X), Cell::Played(Player::O)],
                vec![Cell::Played(Player::O), Cell::Played(Player::X), Cell::Played(Player::O)],
            ],
        };

        let mut board_state = BoardState::new(Player::X, board, TimeSetting::Infinite, Some(3));
        assert_eq!(board_state.get_winner(), None, "There should be no winner in a draw.");
    }

    #[test]
    fn test_get_winner_ongoing_game_no_winner() {
        // Game ongoing with no winner yet
        let board = Board {
            rows: vec![
                vec![Cell::Playable, Cell::Playable, Cell::Playable],
                vec![Cell::Playable, Cell::Played(Player::X), Cell::Playable],
                vec![Cell::Playable, Cell::Playable, Cell::Playable],
            ],
        };

        let mut board_state = BoardState::new(Player::X, board, TimeSetting::Infinite, Some(3));
        assert_eq!(board_state.get_winner(), None, "There should be no winner as the game is ongoing.");
    }
}

pub struct MoveTokenParser;
pub const MOVE: &str = "move";
generate_token_parser!(MOVE, MoveTokenParser);

pub struct WinLengthTokenParser;
pub const WINL: &str = "win-length";
generate_token_parser!(WINL, WinLengthTokenParser);

pub type WinLengthParser = And3<WinLengthTokenParser, WhiteSpaceParser, Number>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Playable,
    NonPlayable,
    Played(Player),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub enum Player {
    X = 1,
    O = -1,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match *self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    rows: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(rows: Vec<Vec<Cell>>) -> Self {
        Self { rows }
    }

    pub fn get_rows(&self) -> Vec<Vec<Cell>> {
        self.rows.clone()
    }

    pub fn play_move(&mut self, x: u32, y: u32, player: Player) {
        if let Some(row) = self.rows.get_mut(x as usize) {
            if let Some(cell) = row.get_mut(y as usize) {
                *cell = Cell::Played(player);
            }
        }
    }

    pub fn check_win(&self, player: Player, win_length: u32) -> bool {
        let rows = self.get_rows();
        let rlen = rows.len();
        let clen = rows[0].len();

        // Check rows
        for row in rows {
            let mut count = 0;
            for cell in row {
                if cell == Cell::Played(player.clone()) {
                    count += 1;
                    if count >= win_length {
                        return true;
                    }
                } else {
                    count = 0;
                }
            }
        }

        // Check columns
        for col in 0..clen {
            let mut count = 0;
            for row in 0..rlen {
                if self.rows[row][col] == Cell::Played(player.clone()) {
                    count += 1;
                    if count >= win_length {
                        return true;
                    }
                } else {
                    count = 0;
                }
            }
        }

        // Check diagonals
        for row in 0..rlen {
            for col in 0..clen {
                if self.rows[row][col] == Cell::Played(player.clone()) {
                    // Check diagonal down-right
                    if row + win_length as usize <= rlen && col + win_length as usize <= clen {
                        let mut count = 1;
                        for i in 1..win_length as usize {
                            if self.rows[row + i][col + i] == Cell::Played(player.clone()) {
                                count += 1;
                            } else {
                                break;
                            }
                        }
                        if count >= win_length as usize {
                            return true;
                        }
                    }

                    // Check diagonal up-right
                    if row >= (win_length as usize - 1) && col + win_length as usize <= clen {
                        let mut count = 1;
                        for i in 1..win_length as usize {
                            if self.rows[row - i][col + i] == Cell::Played(player.clone()) {
                                count += 1;
                            } else {
                                break;
                            }
                        }
                        if count >= win_length as usize {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
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

pub type MoveWithInfiniteTime = And2<BasicMoveParser, AnythingParser>;
pub type MoveWithTotalTime = And3<BasicMoveParser, WhiteSpaceParser, TotalTimeParser>;
pub type MoveWithTimeRemaining = And3<BasicMoveParser, WhiteSpaceParser, TimeRemainingParser>;

pub type MoveParser = And2<
    Or3<MoveWithTotalTime, MoveWithTimeRemaining, MoveWithInfiniteTime>,
    Or2<And2<WhiteSpaceParser, WinLengthParser>, NothingParser>,
>;
pub type MoveParserReturnType = (
    Either<
        (
            (String, (String, (Board, Player))),
            (String, (String, (String, Number))),
        ),
        Either<
            (
                (String, (String, (Board, Player))),
                (String, (String, (String, Number))),
            ),
            ((String, (String, (Board, Player))), Anything),
        >,
    >,
    Either<(String, (String, (String, Number))), Nothing>,
);

impl From<MoveParserReturnType> for BoardState {
    fn from(value: MoveParserReturnType) -> Self {
        match value {
            (Either::Left(((_, (_, (board, player))), (_, (_, (_, time))))), win_length) => {
                BoardState::new(
                    player,
                    board,
                    TimeSetting::TotalTime(time),
                    win_length.map_left(|(_, (_, (_, number)))| number.0).left(),
                )
            }
            (
                Either::Right(Either::Left(((_, (_, (board, player))), (_, (_, (_, time)))))),
                win_length,
            ) => BoardState::new(
                player,
                board,
                TimeSetting::TimeRemaining(time),
                win_length.map_left(|(_, (_, (_, number)))| number.0).left(),
            ),

            (Either::Right(Either::Right(((_, (_, (board, player))), _))), win_length) => {
                BoardState::new(
                    player,
                    board,
                    TimeSetting::Infinite,
                    win_length.map_left(|(_, (_, (_, number)))| number.0).left(),
                )
            }
        }
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

    use crate::parser::{anything_parser::Anything, nothing_parser::Nothing, Parser};

    use super::*;

    use Cell::{Playable, Played};

    #[test]
    fn parse_move_without_time() {
        let move_string = String::from("move 3_/_x_/3_ o");

        let res = MoveParser::parse_from(&move_string);

        assert_eq!(
            res,
            Ok((
                (
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
                        Anything::new("".to_string())
                    ))),
                    Right(Nothing)
                ),
                String::from(""),
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
                (
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
                    Right(Nothing)
                ),
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
                (
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
                    Right(Nothing)
                ),
                "".to_string()
            ))
        );
    }

    #[test]
    fn parse_move_with_win_length() {
        let move_string = String::from("move 3_/3_/3_ x win-length 3");

        let res = MoveParser::parse_from(&move_string);

        assert_eq!(
            res,
            Ok((
                (
                    Right(Right((
                        ((
                            String::from("move"),
                            (
                                String::from(" "),
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
                        Anything::new(" win-length 3".to_string())
                    ))),
                    Left((
                        " ".to_string(),
                        ("win-length".to_string(), (" ".to_string(), Number(3)))
                    ))
                ),
                String::from(""),
            ))
        );
    }
}

#[cfg(test)]
mod test_best_move {
    use crate::parser::move_parser::BestMove;

    #[test]
    fn test_column_notation() {
        let best_move = BestMove::new(0, 0);
        assert_eq!("a", best_move.get_column_notation());
    }
}
