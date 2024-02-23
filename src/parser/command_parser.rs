use either::Either;

use super::{
    identify_parser::{Identify, IdentifyParser, IDENTIFY},
    move_parser::{BoardState, MoveParser, MOVE},
    parse_result::ParseResult,
    parser::Parser,
    quit_parser::{QuitParser, QUIT},
    step_parser::{Step, StepParser, ST3P},
    time_parser::TimeSetting,
    version_parser::Version,
};

pub enum Command {
    Step,
    Identify,
    Move,
    Quit,
}

pub struct CommandParser;

impl Parser<Command> for CommandParser {
    fn parse_from(val: &String) -> ParseResult<Command> {
        match val {
            s if s.starts_with(ST3P) => Ok((Command::Step, val.to_string())),
            s if s.starts_with(IDENTIFY) => Ok((Command::Identify, val.to_string())),
            s if s.starts_with(MOVE) => Ok((Command::Move, val.to_string())),
            s if s.starts_with(QUIT) => Ok((Command::Quit, val.to_string())),
            _ => Err(String::from("Invalid Command")),
        }
    }
}

impl Command {
    pub fn execute(&self, val: &String) -> Result<(), String> {
        match *self {
            Command::Step => {
                let res = StepParser::parse_from(val)?;
                let ((_, (_, (_, (_, version)))), _) = res;

                println!("{}", Step(Version(version)));
            }
            Command::Identify => {
                let _ = IdentifyParser::parse_from(val)?;
                println!("{}", Identify::new());
            }
            Command::Move => {
                let res = MoveParser::parse_from(val)?;

                let move_command: BoardState = match res.0 {
                    Either::Left((((_, (_, board)), player), (_, (_, (_, time))))) => BoardState {
                        player_to_move: player,
                        board,
                        time_setting: TimeSetting::TotalTime(time),
                    },
                    Either::Right(Either::Left((
                        ((_, (_, board)), player),
                        (_, (_, (_, time))),
                    ))) => BoardState {
                        player_to_move: player,
                        board,
                        time_setting: TimeSetting::TimeRemaining(time),
                    },

                    Either::Right(Either::Right((((_, (_, board)), player), _))) => BoardState {
                        player_to_move: player,
                        board,
                        time_setting: TimeSetting::Infinite,
                    },
                };

                println!("{}", move_command.best_move());
            }
            Command::Quit => {
                let res = QuitParser::parse_from(val)?;
                res.0.exit_engine();
            }
        };

        Ok(())
    }
}
