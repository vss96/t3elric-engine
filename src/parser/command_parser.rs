use either::Either;

use super::{
    identify_parser::{Identify, IdentifyParser},
    move_parser::{map_to_move, BoardState, MoveParser, MoveParserReturnType},
    or_parser::Or4,
    quit_parser::{Quit, QuitParser},
    step_parser::{Step, StepParser, StepParserReturnType},
    time_parser::TimeSetting,
    version_parser::Version,
};

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Step(Step),
    Identify(Identify),
    Move(BoardState),
    Quit(Quit),
}

pub type CommandParser = Or4<StepParser, IdentifyParser, MoveParser, QuitParser>;
pub type ComandParserReturnType =
    Either<StepParserReturnType, Either<String, Either<MoveParserReturnType, String>>>;

pub fn map_to_command(parser_output: ComandParserReturnType) -> Command {
    return match parser_output {
        Either::Left(output) => {
            let (_, (_, (_, (_, version)))) = output;

            Command::Step(Step(Version(version)))
        }
        Either::Right(Either::Left(_)) => Command::Identify(Identify::new()),
        Either::Right(Either::Right(Either::Left(output))) => Command::Move(map_to_move(output)),
        Either::Right(Either::Right(Either::Right(_))) => Command::Quit(Quit {}),
    };
}

impl Command {
    pub fn execute(&self) -> Result<(), String> {
        match self {
            Command::Step(step) => {
                println!("{}", step);
            }
            Command::Identify(identify) => {
                println!("{}", identify);
            }
            Command::Move(board_state) => {
                // println!("{}", board_state.best_move());
            }
            Command::Quit(quit) => {
                quit.exit_engine();
            }
        };

        Ok(())
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

    use super::{map_to_command, CommandParser};

    #[test]
    fn test_move() {
        let move_string = String::from("move 3_/_x_/3_ o");
        let val = &move_string.clone();
        let command = CommandParser::parse_from(val).map(|(res, _)| map_to_command(res));
        assert_eq!(
            command,
            Ok(Command::Move(BoardState {
                player_to_move: Player::O,
                board: Board {
                    rows: vec![
                        vec![Playable, Playable, Playable],
                        vec![Playable, Played(Player::X), Playable],
                        vec![Playable, Playable, Playable]
                    ]
                },
                time_setting: TimeSetting::Infinite
            }))
        );
    }
}
