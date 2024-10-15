use either::Either;

use crate::{
    parser::{Command, CommandResponse, Identity, Quit, Step},
    solver::Solver,
};

pub struct CommandExecutor {
    solver: Box<dyn Solver>,
}

impl CommandExecutor {
    pub fn new(solver: Box<dyn Solver>) -> Self {
        Self { solver }
    }

    pub fn execute(&self, command: Command) -> Either<CommandResponse, Quit> {
        match command {
            Command::Init(version) => Either::Left(CommandResponse::StepOk(Step::new(version))),
            Command::Identify => Either::Left(CommandResponse::Identity(Identity::new())),
            Command::Move(board_state) => Either::Left(CommandResponse::Play(
                self.solver.solve(&board_state).0.unwrap(),
            )),
            Command::Quit => Either::Right(Quit {}),
        }
    }
}
