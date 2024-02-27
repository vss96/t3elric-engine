use crate::generate_token_parser;

use super::{
    and_parser::And3, number_parser::Number, whitespace_parser::WhiteSpaceParser, ParseResult,
    Parser,
};

#[derive(PartialEq, Eq, Debug)]
pub struct Version(pub Number);
const VERSION: &str = "version";

pub struct VersionNumberParser;
pub struct VersionTokenParser;

generate_token_parser!(VERSION, VersionTokenParser);

impl Parser<Number> for VersionNumberParser {
    fn parse_from(val: &String) -> ParseResult<Number> {
        Number::try_from(val).map(|n| (n, "".to_string()))
    }
}

pub type VersionParser = And3<VersionTokenParser, WhiteSpaceParser, VersionNumberParser>;

#[cfg(test)]
mod test_version_token_parser {
    use crate::parser::{version_parser::VERSION, Parser};

    use super::VersionTokenParser;

    #[test]
    fn parse_version_token() {
        let version_string = "version 1".to_string();
        let res = VersionTokenParser::parse_from(&version_string);
        assert_eq!(Ok((VERSION.to_string(), " 1".to_string())), res);
    }

    #[test]
    fn error_invalid_token() {
        let invalid_version_string = "v 1.1.1".to_string();
        let res = VersionTokenParser::parse_from(&invalid_version_string);
        assert_eq!(Err(String::from("Could not find token: version")), res);
    }
}

#[cfg(test)]
mod test_version_number_parser {
    use super::Number;
    use crate::parser::{version_parser::VersionNumberParser, Parser};

    #[test]
    fn parse_version_number() {
        let version_number = "1".to_string();
        let res = VersionNumberParser::parse_from(&version_number);
        assert_eq!(Ok((Number(1), "".to_string())), res);
    }

    #[test]
    fn error_for_invalid_number() {
        let version_number = "1x".to_string();
        let res = VersionNumberParser::parse_from(&version_number);
        assert_eq!(Err("x is not a digit".to_string()), res);
    }
}

#[cfg(test)]
mod test_version_parser {
    use crate::parser::{number_parser::Number, Parser};

    use super::{VersionParser, VERSION};

    #[test]
    fn parse_version() {
        let version_string = "version 2".to_string();
        let res = VersionParser::parse_from(&version_string);
        assert_eq!(
            Ok((
                (VERSION.to_string(), (" ".to_string(), Number(2))),
                "".to_string()
            )),
            res
        );
    }
}
