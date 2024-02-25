use super::{ParseResult, Parser};
use either::Either;

pub struct OrParser<P1 , P2>(P1, P2);


impl<P1, P2, U, V> Parser<Either<U, V>> for OrParser<P1, P2>
where
    P1: Parser<U>,
    P2: Parser<V>,
{
    fn parse_from(val: &String) -> ParseResult<Either<U, V>> {
        P1::parse_from(val)
            .map(|v| (Either::Left(v.0), v.1))
            .or_else(|_| P2::parse_from(val).map(|v| (Either::Right(v.0), v.1)))
    }
}

pub type TriOr<U, V, W> = OrParser<U, OrParser<V, W>>;

#[cfg(test)]
mod test_token_parser {
    use crate::parser::{ParseResult, Parser, or_parser::Either};

    use super::OrParser;

    struct DummyParser1;

    impl Parser<String> for DummyParser1 {
        fn parse_from(val: &String) -> ParseResult<String> {
            if val.contains("dummy1") {
                return Ok(("dummy1".to_string(), " data".to_string()));
            }

            Err("Not a valid dummy1".to_string())
        }
    }

    struct DummyParser2;

    impl Parser<u32> for DummyParser2 {
        fn parse_from(val: &String) -> ParseResult<u32> {
            if val.contains("dummy2") {
                return Ok((2, "".to_string()));
            }

            Err("Not a valid dummy2".to_string())
        }
    }

    #[test]
    fn test_or_parser_first_parser_succeeds() {
        let input = String::from("dummy1 data");

        let result = OrParser::<DummyParser1, DummyParser2>::parse_from(&input);

        assert_eq!(result, Ok((Either::Left("dummy1".to_string()), " data".to_string())));
    }

    #[test]
    fn test_or_parser_second_parser_succeeds() {
        let input = String::from("dummy2 data");

        let result = OrParser::<DummyParser1, DummyParser2>::parse_from(&input);

        assert_eq!(result, Ok((Either::Right(2), "".to_string())));
    }
}