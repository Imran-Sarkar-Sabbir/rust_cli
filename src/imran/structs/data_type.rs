use std::fmt::Debug;

use serde_json::{Map, Value};

use crate::imran::lib::parse_value::{parse_bool, parse_number, parse_string};

#[derive(Debug)]
pub struct DataType {
    d_type: String,
    required: bool,
    min: Option<i64>,
    max: Option<i64>,
    col: Option<i64>,
    row: Option<i64>,
}

impl DataType {
    pub fn from_json(json_data: &Map<String, Value>) -> DataType {
        DataType {
            d_type: parse_string(json_data, "type").unwrap().to_string(),
            required: parse_bool(json_data, "required").unwrap(),
            min: parse_number(json_data, "min"),
            max: parse_number(json_data, "max"),
            col: parse_number(json_data, "col"),
            row: parse_number(json_data, "row"),
        }
    }
}
