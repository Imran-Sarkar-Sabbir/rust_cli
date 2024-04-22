use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::imran::lib::parse_value::parse_string;

use super::{default_config::DefaultConfig, property::Property};

#[derive(Debug)]
pub struct Config {
    name: String,
    properties: HashMap<String, Property>,
}

impl Config {
    pub fn from_json(default_config: DefaultConfig, config: Map<String, Value>) {
        let name = parse_string(&config, "name");

        let mut properties: HashMap<String, Property> = HashMap::new();

        match config.get("properties") {
            Some(json_properties) => {
                for (key, val) in json_properties.as_object().unwrap().iter() {
                    properties.insert(
                        key.to_string(),
                        merge_default_properties(val.as_object().unwrap(), default_config),
                    );
                }
            }
            None => {}
        }
    }
}

fn merge_default_properties(
    prop_config: &Map<String, Value>,
    default_config: DefaultConfig,
) -> Property {
    let prop_value = parse_string(prop_config, "value").unwrap().to_string();

    Property {
        message_key: parse_string(prop_config, "message_key").unwrap().to_string(),
        value: 
    }
}
