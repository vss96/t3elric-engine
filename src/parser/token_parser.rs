#[macro_export]
macro_rules! generate_token_parser {
    ($token:expr, $name:ident) => {
        impl Parser<String> for $name {
            fn parse_from(val: &String) -> ParseResult<String> {
                if val.starts_with($token) {
                    Ok(($token.to_string(), val[$token.len()..].to_string()))
                } else {
                    Err(format!("Could not find token: {}", $token))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_generic_token_parser {
    use crate::parser::parse_result::ParseResult;
    use crate::parser::Parser;

    #[test]
    pub fn test_dummy_token_parser() {
        struct DummyTokenParser;
        generate_token_parser!("dummy", DummyTokenParser);
        let dummy_string = "dummy 123".to_string();

        let parse_result = DummyTokenParser::parse_from(&dummy_string);

        assert_eq!(parse_result, Ok(("dummy".to_string(), " 123".to_string())));
    }
}
