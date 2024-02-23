use super::{ParseResult, Parser};

#[derive(PartialEq, Eq, Debug)]
pub struct Nothing;

pub struct NothingParser;

impl Parser<Nothing> for NothingParser {
    fn parse_from(val: &String) -> ParseResult<Nothing> {
        if val.is_empty() {
            return Ok((Nothing {}, String::from("")));
        }
        Err(format!("Expected nothing, found string : {}", val))
    }
}
