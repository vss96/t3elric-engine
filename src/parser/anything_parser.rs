use super::{ParseResult, Parser};

pub struct AnythingParser;

#[derive(PartialEq, Eq, Debug)]
struct Anything {
    value: String,
}

impl Parser<Anything> for AnythingParser {
    fn parse_from(val: &String) -> ParseResult<Anything> {
        Ok((Anything { value: val.clone() }, String::from("")))
    }
}
