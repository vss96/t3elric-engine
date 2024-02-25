use std::{collections::HashMap, fmt::Display};

use crate::generate_token_parser;

use super::{ParseResult, Parser};

#[derive(PartialEq, Eq, Debug)]
pub struct Identify {
    engine_info: HashMap<String, String>,
}

pub struct IdentifyParser;

pub const IDENTIFY: &str = "identify";

generate_token_parser!(IDENTIFY, IdentifyParser);

impl Identify {
    pub fn new() -> Self {
        let mut engine_info = HashMap::new();
        engine_info.insert("name".to_string(), "t3elric-engine".to_string());
        engine_info.insert("author".to_string(), "shettyvikas209@gmail.com".to_string());
        engine_info.insert("version".to_string(), "1.0.0".to_string());
        engine_info.insert(
            "url".to_string(),
            "https://github.com/vss96/t3elric-engine".to_string(),
        );

        Identify { engine_info }
    }
}

impl Display for Identify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.engine_info {
            write!(f, "identify {} {}\n", key, value)?;
        }
        write!(f, "identify ok\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod test_identify_token_parser {

    use crate::parser::{identify_parser::IDENTIFY, Parser};

    use super::IdentifyParser;

    #[test]
    fn parse_version_token() {
        let identify_string = "identify".to_string();
        let res = IdentifyParser::parse_from(&identify_string);
        assert_eq!(Ok((IDENTIFY.to_string(), "".to_string())), res);
    }

    #[test]
    fn error_invalid_token() {
        let invalid_identify_string = "identifx".to_string();
        let res = IdentifyParser::parse_from(&invalid_identify_string);
        assert_eq!(Err(String::from("Could not find token: identify")), res);
    }
}
