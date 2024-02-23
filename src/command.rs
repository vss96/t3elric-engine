enum Command {
Step,
Identify,
Move,
Quit
}

impl Command {

    pub fn value_of(val: &str) -> Option<Self> {
        match val {
            "st3p" => Some(Stage::Step),
            "identify" => Some(Stage::Identify),
            "move" => Some(Stage::Move),
            "quit" => Some(Stage::Quit),
            _ => None,
        }
    }
}