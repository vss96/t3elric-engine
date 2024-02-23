
use super::{ParseResult, Parser};
pub struct WhiteSpaceParser;


const WHITESPACE: &str = " ";

impl Parser<String> for WhiteSpaceParser {
    fn parse_from(val: &String) -> ParseResult<String>  {
        if val.starts_with(WHITESPACE) {
            return Ok((WHITESPACE.to_string(), val[WHITESPACE.len()..].to_string()));
        }

        return Err("Could not find whitespace.".to_string())
    }
}

mod whitespace_parser_test {
    use crate::parser::{whitespace_parser::{WhiteSpaceParser, WHITESPACE}, Parser};



    

#[test]
fn should_parse_string_starting_with_whitespace() {
    let val = " 234".to_string();
    assert_eq!(Ok((WHITESPACE.to_string(), "234".to_string())), WhiteSpaceParser::parse_from(&val));
}

}