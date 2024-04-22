use serde_json::{Map, Value};

pub fn parse_number(json_data: &Map<String, Value>, key: &str) -> Option<i64> {
    match json_data.get(key) {
        Some(value) => return value.as_i64(),
        None => None,
    }
}

pub fn parse_string<'a>(json_data: &'a Map<String, Value>, key: &'a str) -> Option<&'a str> {
    match json_data.get(key) {
        Some(value) => return value.as_str(),
        None => None,
    }
}

pub fn parse_bool(json_data: &Map<String, Value>, key: &str) -> Option<bool> {
    match json_data.get(key) {
        Some(value) => return value.as_bool(),
        None => None,
    }
}
