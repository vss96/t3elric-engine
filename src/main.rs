use std::io::{stdin, Result};

mod evaluator;
mod executor;
mod parser;
mod solver;
mod scorer;
use executor::CommandExecutor;
use parser::Command;
use scorer::GreedyScorer;
use solver::{GreedySolver, LookAheadSolver};
use std::time::Instant; 
fn main() -> Result<()> {
    let my_solver = LookAheadSolver::new(Box::new(GreedyScorer::default()), 1);
    // let my_solver = GreedySolver::default();
    let executor = CommandExecutor::new(Box::new(my_solver));
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        let input_string = buffer.trim();
        let start = Instant::now();
        let res = input_string.parse::<Command>();
        // println!("Parse time: {:?}", start.elapsed());

        match res {
            Ok(command) => {
                executor
                    .execute(command)
                    .map_either(|f| println!("{}", f), |g| g.exit_engine());
            }
            Err(msg) => eprintln!("Invalid input: {} || msg: {}", input_string, msg),
        };


        // println!("Total time: {:?}", start.elapsed());
    }
}
