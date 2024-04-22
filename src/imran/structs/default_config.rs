use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::imran::structs::data_type::DataType;

#[derive(Debug)]
pub struct DefaultConfig {
    pattern: HashMap<String, String>,
    data_types: HashMap<String, DataType>,
}

impl DefaultConfig {
    pub fn from_map(default_config: Map<String, Value>) -> DefaultConfig {
        let mut pattern: HashMap<String, String> = HashMap::new();
        let mut data_types: HashMap<String, DataType> = HashMap::new();

        println!("from_map called");

        match default_config.get("pattern") {
            Some(json_pattern) => {
                for (key, val) in json_pattern.as_object().unwrap().iter() {
                    pattern.insert(key.clone(), val.as_str().unwrap().to_string());
                }
            }
            None => {}
        }

        match default_config.get("data_types") {
            None => {}
            Some(value) => {
                for (key, val) in value.as_object().unwrap().iter() {
                    data_types.insert(key.clone(), DataType::from_json(val.as_object().unwrap()));
                }
            }
        };
        println!("{:?}", data_types);
        DefaultConfig {
            pattern: pattern,
            data_types: data_types,
        }
    }
}
