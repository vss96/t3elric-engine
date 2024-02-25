use crate::generate_token_parser;
use crate::parser::parse_result::ParseResult;
use crate::parser::Parser;

const WHITESPACE: &str = " ";
pub struct WhiteSpaceParser;
generate_token_parser!(WHITESPACE, WhiteSpaceParser);

#[cfg(test)]
mod whitespace_parser_test {
    use crate::parser::{
        whitespace_parser::{WhiteSpaceParser, WHITESPACE},
        Parser,
    };

    #[test]
    fn should_parse_string_starting_with_whitespace() {
        let val = " 234".to_string();
        assert_eq!(
            Ok((WHITESPACE.to_string(), "234".to_string())),
            WhiteSpaceParser::parse_from(&val)
        );
    }
}
