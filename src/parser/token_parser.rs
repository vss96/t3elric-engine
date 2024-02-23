use super::ParseResult;

pub trait TokenParser<T> {
    fn parse_from(&self, val: &String) -> ParseResult<T>;
}

pub struct GenericTokenParser {
    token: String,
}

impl TokenParser<String> for GenericTokenParser {
    fn parse_from(&self, val: &String) -> ParseResult<String> {
        if val.starts_with(&self.token) {
            return Ok((self.token.clone(), val[self.token.len()..].to_string()));
        }

        Err("Invalid token type".to_string())
    }
}

#[cfg(test)]
mod test_generic_token_parser {
    use super::{GenericTokenParser, TokenParser};

    #[test]
    pub fn parse_dummy_token() {
        let dummy_token = GenericTokenParser {
            token: String::from("dummy"),
        };
        let dummy_string = "dummy 123".to_string();

        let parse_result = dummy_token.parse_from(&dummy_string);

        assert_eq!(parse_result, Ok(("dummy".to_string(), " 123".to_string())));
    }
}
