use std::fmt::Debug;

use serde_json::{Map, Value};

#[derive(Debug)]
pub struct DataType {
    d_type: String,
    required: bool,
    min: Option<i64>,
    max: Option<i64>,
    col: Option<i64>,
    row: Option<i64>,
}

fn parse_number(json_data: &Map<String, Value>, key: String) -> Option<i64> {
    match json_data.get(key.as_str()) {
        Some(value) => return value.as_i64(),
        None => None,
    }
}

impl DataType {
    pub fn from_json(json_data: &Map<String, Value>) -> DataType {
        DataType {
            d_type: json_data.get("type").unwrap().as_str().unwrap().to_string(),
            required: json_data.get("required").unwrap().as_bool().unwrap(),
            min: parse_number(json_data, "min".to_string()),
            max: parse_number(json_data, "max".to_string()),
            col: parse_number(json_data, "col".to_string()),
            row: parse_number(json_data, "row".to_string()),
        }
    }
}
