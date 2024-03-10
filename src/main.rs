use std::io::{stdin, Result};

mod evaluator;
mod executor;
mod parser;
mod solver;
use executor::CommandExecutor;
use parser::{Command, CommandParser, Parser};
use solver::GreedySolver;

fn main() -> Result<()> {
    let my_solver = GreedySolver::default();
    let executor = CommandExecutor::new(Box::new(my_solver));
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        let input_string = buffer.trim().to_owned();

        let res = CommandParser::parse_from(&input_string)
            .map(Command::from);

        match res {
            Ok(command) => 
               { executor
                .execute(command)
                .map_either(|f| println!("{}", f), |g| g.exit_engine());
               }
            ,
            Err(_) => 
                eprintln!("Invalid input: {}", input_string)
            
        };
    }
}
