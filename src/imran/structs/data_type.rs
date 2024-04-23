use std::fmt::Debug;

use serde_json::{Map, Value};

use crate::imran::lib::parse_value::{parse_bool, parse_number, parse_string};

#[derive(Debug)]
pub struct DataType {
    pub d_type: String,
    pub required: bool,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub col: Option<i64>,
    pub row: Option<i64>,
    pub default: Option<Value>,
}

impl DataType {
    pub fn new() -> DataType {
        DataType {
            d_type: "none".to_string(),
            required: false,
            min: None,
            max: None,
            col: None,
            row: None,
            default: None
        }
    }

    pub fn copy_from(&mut self, other: &DataType) {
        self.d_type = other.d_type.clone();
        self.required = other.required;
        self.min = other.min;
        self.max = other.max;
        self.col = other.col;
        self.row = other.row;
        self.default = other.default.clone();
    }

    pub fn from_json(json_data: &Map<String, Value>) -> DataType {
        let v= json_data.get("default").clone();
        let mut default_val = None;
        match v {
            None => {}
            Some(res) => {
                default_val = Some(res.clone());
            }
        }
        DataType {
            d_type: parse_string(json_data, "type").unwrap().to_string(),
            required: parse_bool(json_data, "required").unwrap(),
            min: parse_number(json_data, "min"),
            max: parse_number(json_data, "max"),
            col: parse_number(json_data, "col"),
            row: parse_number(json_data, "row"),
            default: default_val,
        }
    }
}
