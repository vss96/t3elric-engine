use super::{ParseResult, Parser};

pub struct AndParser<P1, P2> (P1, P2);

impl<P1, P2, X, Y> Parser<(X, Y)> for AndParser<P1, P2>
where
    P1: Parser<X>,
    P2: Parser<Y>,
{
    fn parse_from(val: &String) -> ParseResult<(X, Y)> {
        P1::parse_from(val)
        .and_then(|(x, follow)| {
            P2::parse_from(&follow)
            .map(|(y, remaining)| ((x, y), remaining))
        })
    }
}


pub type TriAnd<U, V, W > = AndParser< U, AndParser<V, W> >;