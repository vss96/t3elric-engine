use std::io::{stdin, Result};

mod parser;
mod solver;
use parser::{map_to_command, CommandParser, Parser};

fn main() -> Result<()> {
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        let input_string = buffer.trim().to_owned();
        let command =
            CommandParser::parse_from(&input_string).map(|(output, _)| map_to_command(output));

        let _ = command.unwrap().execute();
    }
}
