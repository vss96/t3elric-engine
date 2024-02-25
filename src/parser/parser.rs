use super::ParseResult;
pub trait Parser<T> {
    fn parse_from(val: &String) -> ParseResult<T>;
}
