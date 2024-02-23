use std::fmt::Display;

use super::{
    and_parser::TriAnd, version_parser::{Version, VersionParser}, whitespace_parser::WhiteSpaceParser, ParseResult, Parser
};
pub struct Step(pub Version);

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} version {} ok", ST3P, self.0.0)
    }
}

pub struct StepTokenParser;

pub const ST3P: &str = "st3p";

impl Parser<String> for StepTokenParser {
    fn parse_from(val: &String) -> ParseResult<String> {
        if val.starts_with(ST3P) {
            return Ok((ST3P.to_string(), val[ST3P.len()..].to_string()));
        }

        Err("Invalid step type".to_string())
    }
}

pub type StepParser = TriAnd<StepTokenParser, WhiteSpaceParser, VersionParser>;

#[cfg(test)]
mod test_step_parser {
    use crate::parser::{
        number_parser::Number,
        step_parser::{StepParser, ST3P},
        Parser,
    };

    #[test]
    fn parse_step() {
        let version_string = "st3p version 1".to_string();
        let res = StepParser::parse_from(&version_string);
        assert_eq!(
            (
                Ok(((ST3P.to_string(),(" ".to_string(),
                 ("version".to_string(), (" ".to_string(), Number(1)))) ), "".to_string()))
            ),
            res
        );
    }
}
