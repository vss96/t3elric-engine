use std::fmt::Display;

use crate::generate_token_parser;

use super::{
    and_parser::And3,
    number_parser::Number,
    version_parser::{Version, VersionParser},
    whitespace_parser::WhiteSpaceParser,
    ParseResult, Parser,
};

#[derive(PartialEq, Eq, Debug)]
pub struct Step(pub Version);

impl Step {
    pub fn new(version: Number) -> Self {
        Self(Version(version))
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} version {}", ST3P, self.0 .0)
    }
}

pub struct StepTokenParser;

pub const ST3P: &str = "st3p";

generate_token_parser!(ST3P, StepTokenParser);

pub type StepParser = And3<StepTokenParser, WhiteSpaceParser, VersionParser>;
pub type StepParserReturnType = (String, (String, (String, (String, Number))));

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
            (Ok((
                (
                    ST3P.to_string(),
                    (
                        " ".to_string(),
                        ("version".to_string(), (" ".to_string(), Number(1)))
                    )
                ),
                "".to_string()
            ))),
            res
        );
    }
}
