use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::imran::structs::data_type::DataType;

#[derive(Debug)]
pub struct DefaultConfig {
    pattern: HashMap<String, String>,
    data_types: HashMap<String, DataType>,
}

impl DefaultConfig {
    pub fn from_map(default_config: Map<String, Value>) {
        let pattern = default_config.get("pattern");
        let mut data_types: HashMap<String, DataType> = HashMap::new();

        println!("from_map called");

        match default_config.get("data_types") {
            None => {}
            Some(value) => {
                for (key, val) in value.as_object().unwrap().iter() {
                    data_types.insert(key.clone(), DataType::from_json(val.as_object().unwrap()));
                }
            }
        };
    }
}