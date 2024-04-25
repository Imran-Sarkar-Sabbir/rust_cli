use serde_json::{Map, Value};

use crate::imran::lib::parse_value::parse_string;
#[derive(Debug)]
pub struct Name {
    pub upper: String,
    pub capital: String,
    pub camel: String,
    pub snake: String,
    pub kabeb: String,
}

impl Name {
    pub fn from_json(config: &Map<String, Value>) -> Name {
        Name {
            upper: parse_string(config, "upper").unwrap().to_string(),
            capital: parse_string(config, "capital").unwrap().to_string(),
            camel: parse_string(config, "camel").unwrap().to_string(),
            snake: parse_string(config, "snake").unwrap().to_string(),
            kabeb: parse_string(config, "kabeb").unwrap().to_string(),
        }
    }
}