use super::{ParseResult, Parser};

pub struct AnythingParser;

#[derive(PartialEq, Eq, Debug)]
pub struct Anything {
    value: String,
}

impl Parser<Anything> for AnythingParser {
    fn parse_from(val: &String) -> ParseResult<Anything> {
        Ok((Anything { value: val.clone() }, val.clone()))
    }
}

impl Anything {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
