use super::{
    and_parser::{AndParser, TriAnd}, number_parser::Number, or_parser::OrParser, whitespace_parser::WhiteSpaceParser, ParseResult, Parser
};

pub struct TimeTokenParser;
const TIME: &str = "time";

pub struct TimeRemainingTokenParser;
const TIME_REMAINING: &str = "time-remaining";

impl Parser<String> for TimeTokenParser {
    fn parse_from(val: &String) -> ParseResult<String> {
        if val.starts_with(TIME) {
            return Ok((TIME.to_string(), val[TIME.len()..].to_string()));
        }

        Err("Invalid time type".to_string())
    }
}

impl Parser<String> for TimeRemainingTokenParser {
    fn parse_from(val: &String) -> ParseResult<String> {
        if val.starts_with(TIME_REMAINING) {
            return Ok((
                TIME_REMAINING.to_string(),
                val[TIME_REMAINING.len()..].to_string(),
            ));
        }

        Err("Invalid time type".to_string())
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum TimeSetting{
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
                .map(|(quantity, remaining)| ( quantity , remaining))
                .map_err(|error| format!("Error parsing time: {}", error));
        }

        Err("Invalid time unit".to_string())
    }
}

pub type TotalTimeParser = TriAnd<TimeTokenParser, WhiteSpaceParser,  TimeParser>;
pub type TimeRemainingParser = TriAnd<TimeRemainingTokenParser, WhiteSpaceParser,  TimeParser>;

pub type T3TimeParser = OrParser<TotalTimeParser, TimeRemainingParser>;


mod test_time_parser {
    use crate::parser::{number_parser::Number, Parser};

    use super::TotalTimeParser;



    #[test]
    fn parse_valid_time() {
        let time_string = String::from("time ms:1000");
        let res = TotalTimeParser::parse_from(&time_string);
        assert_eq!(res, Ok((("time".to_string(), (" ".to_string(), Number(1000) )), "".to_string())));
    }
}
