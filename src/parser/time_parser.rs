use crate::generate_token_parser;

use super::{
    and_parser::And3, number_parser::Number, whitespace_parser::WhiteSpaceParser, ParseResult,
    Parser,
};

const TIME: &str = "time";
pub struct TimeTokenParser;
generate_token_parser!(TIME, TimeTokenParser);

pub struct TimeRemainingTokenParser;
const TIME_REMAINING: &str = "time-remaining";
generate_token_parser!(TIME_REMAINING, TimeRemainingTokenParser);

#[derive(PartialEq, Eq, Debug)]
pub enum TimeSetting {
    TotalTime(Number),
    TimeRemaining(Number),
    Infinite,
}

pub struct TimeParser;

const MILLIS: &str = "ms";

impl Parser<Number> for TimeParser {
    fn parse_from(val: &String) -> ParseResult<Number> {
        if val.starts_with(MILLIS) {
            return Number::parse_from(&val[MILLIS.len() + 1..].to_string())
                .map(|(quantity, remaining)| (quantity, remaining))
                .map_err(|error| format!("Error parsing time: {}", error));
        }

        Err("Invalid time unit".to_string())
    }
}

pub type TotalTimeParser = And3<TimeTokenParser, WhiteSpaceParser, TimeParser>;
pub type TimeRemainingParser = And3<TimeRemainingTokenParser, WhiteSpaceParser, TimeParser>;

mod test_time_parser {
    use crate::parser::{number_parser::Number, Parser};

    use super::TotalTimeParser;

    #[test]
    fn parse_valid_time() {
        let time_string = String::from("time ms:1000");
        let res = TotalTimeParser::parse_from(&time_string);
        assert_eq!(
            res,
            Ok((
                ("time".to_string(), (" ".to_string(), Number(1000))),
                "".to_string()
            ))
        );
    }
}
