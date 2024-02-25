use std::fmt::Display;

use super::{Digit, ParseResult, Parser};

#[derive(PartialEq, Eq, Debug)]
pub struct Number(pub u32);

impl TryFrom<&String> for Number {
    type Error = String;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value
            .chars()
            .map(|c| Digit::try_from(c))
            .try_fold(0, |acc, digit| digit.map(|d| acc * 10 + d as u32))
            .map(|value| Number(value))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parser<Number> for Number {
    fn parse_from(val: &String) -> ParseResult<Number> {
        let digits = val
            .chars()
            .take_while(|c| Digit::is_digit(c))
            .collect::<String>();

        if digits.is_empty() {
            return Err("No digits found".to_string());
        }

        Number::try_from(&digits).map(|number| (number, val[digits.len()..].to_string()))
    }
}

#[cfg(test)]
mod test_number_try_from {
    use super::*;

    #[test]
    fn should_read_a_valid_number_string() {
        let number_string = "1234".to_string();
        assert_eq!(Ok(Number(1234)), Number::try_from(&number_string));
    }

    #[test]
    fn should_not_read_an_invalid_number_string() {
        let number_string = "1234-x".to_string();
        assert_eq!(
            Err(String::from("- is not a digit")),
            Number::try_from(&number_string)
        );
    }

    #[test]
    fn should_read_a_valid_single_digit_string() {
        let number_string = "1".to_string();
        assert_eq!(Ok(Number(1)), Number::try_from(&number_string));
    }
}

#[cfg(test)]
mod test_number_parser {
    use super::*;

    #[test]
    fn should_parse_number_from_string() {
        let number_string = "1234-x".to_string();
        assert_eq!(
            Ok((Number(1234), "-x".to_string())),
            Number::parse_from(&number_string)
        );
    }

    #[test]
    fn err_if_string_has_no_number() {
        let invalid_string = "xyz-".to_string();
        assert_eq!(
            Err("No digits found".to_string()),
            Number::parse_from(&invalid_string)
        );
    }
}
