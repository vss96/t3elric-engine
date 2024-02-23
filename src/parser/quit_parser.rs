use std::process::exit;

use super::{ParseResult, Parser};

#[derive(PartialEq, Eq, Debug)]
pub struct Quit;

impl Quit {


    pub fn exit_engine(&self) {
        exit(0);
    }
}


pub struct QuitParser;




pub const QUIT : &str = "quit";


impl Parser<Quit> for QuitParser {
    fn parse_from(val: &String) -> ParseResult<Quit>  {
        if val.starts_with(QUIT) {
            return Ok((Quit {}, val[QUIT.len()..].to_string()));
        }

        Err("Invalid Quit token".to_string())
    }
}



#[cfg(test)]
mod test_quit_token_parser {

    use crate::parser::{quit_parser::Quit, Parser};

    use super::QuitParser;

    #[test]
    fn parse_version_token() {
        let identify_string = "quit".to_string();
        let res = QuitParser::parse_from(&identify_string);
        assert_eq!(Ok((Quit{}, "".to_string())), res);
    }

    #[test]
    fn error_invalid_token() {
        let invalid_quit_string = "quix".to_string();
        let res = QuitParser::parse_from(&invalid_quit_string);
        assert_eq!(Err(String::from("Invalid Quit token")), res);
    }
}