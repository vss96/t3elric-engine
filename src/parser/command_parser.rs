use std::fmt::{Display, Formatter};

use either::Either;
use std::str::FromStr;

use super::{
    identify_parser::{IdentifyParser, Identity},
    move_parser::{BoardState, MoveParser, MoveParserReturnType},
    number_parser::Number,
    or_parser::Or4,
    quit_parser::QuitParser,
    step_parser::{Step, StepParser, StepParserReturnType},
    BestMove, Parser,
};

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Init(Number),
    Identify,
    Move(BoardState),
    Quit,
}

pub type CommandParser = Or4<StepParser, IdentifyParser, MoveParser, QuitParser>;
pub type ComandParserReturnType = (
    Either<StepParserReturnType, Either<String, Either<MoveParserReturnType, String>>>,
    String,
);

impl From<ComandParserReturnType> for Command {
    fn from(value: ComandParserReturnType) -> Self {
        match value.0 {
            Either::Left(output) => {
                let (_, (_, (_, (_, version)))) = output;

                Command::Init(version)
            }
            Either::Right(Either::Left(_)) => Command::Identify,
            Either::Right(Either::Right(Either::Left(output))) => {
                Command::Move(BoardState::from(output))
            }
            Either::Right(Either::Right(Either::Right(_))) => Command::Quit,
        }
    }
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CommandParser::parse_from(&s.to_string()).map(Command::from)
    }
}

pub enum CommandResponse {
    StepOk(Step),
    Identity(Identity),
    Play(BestMove),
}

impl Display for CommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResponse::StepOk(step) => write!(f, "{} ok", step),
            CommandResponse::Identity(identity) => write!(f, "{}identify ok", identity),
            CommandResponse::Play(best_move) => write!(f, "{}", best_move),
        }
    }
}

#[cfg(test)]
mod test_command {
    use crate::parser::{
        command_parser::Command,
        move_parser::{
            Board, BoardState,
            Cell::{Playable, Played},
            Player,
        },
        time_parser::TimeSetting,
        Parser,
    };

    use super::CommandParser;

    #[test]
    fn test_move() {
        let move_string = String::from("move 3_/_x_/3_ o");
        let val = &move_string.clone();
        let command = CommandParser::parse_from(val).map(Command::from);
        assert_eq!(
            command,
            Ok(Command::Move(BoardState::new(
                Player::O,
                Board::new(vec![
                    vec![Playable, Playable, Playable],
                    vec![Playable, Played(Player::X), Playable],
                    vec![Playable, Playable, Playable]
                ]),
                TimeSetting::Infinite
            )))
        );
    }
}
