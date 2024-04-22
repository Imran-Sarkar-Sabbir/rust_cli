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

enum Type {
    STR,
    NUM,
    BOOL,
}
impl DataType {
    pub fn from_json(json_data: &Map<String, Value>) -> DataType {
        DataType {
            d_type: json_data.unwrap().as_str().unwrap().to_string(),
            required: json_data.get("required").unwrap().as_bool().unwrap(),
            min: json_data.get("min"),
            max: json_data.get("max").unwrap().as_i64(),
            col: json_data.get("col").unwrap().as_i64(),
            row: json_data.get("row").unwrap().as_i64(),
        }
    }
}
