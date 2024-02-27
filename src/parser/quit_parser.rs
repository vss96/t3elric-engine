use std::process::exit;

use crate::generate_token_parser;

use super::{ParseResult, Parser};

#[derive(PartialEq, Eq, Debug)]
pub struct Quit;

impl Quit {
    pub fn exit_engine(&self) {
        exit(0);
    }
}

pub struct QuitParser;

pub const QUIT: &str = "quit";

generate_token_parser!(QUIT, QuitParser);

#[cfg(test)]
mod test_quit_token_parser {

    use crate::parser::{quit_parser::QUIT, Parser};

    use super::QuitParser;

    #[test]
    fn parse_version_token() {
        let identify_string = "quit".to_string();
        let res = QuitParser::parse_from(&identify_string);
        assert_eq!(Ok((QUIT.to_string(), "".to_string())), res);
    }

    #[test]
    fn error_invalid_token() {
        let invalid_quit_string = "quix".to_string();
        let res = QuitParser::parse_from(&invalid_quit_string);
        assert_eq!(Err(String::from("Could not find token: quit")), res);
    }
}
