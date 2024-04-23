use super::{data_type::DataType, default_config::DefaultConfig, property::Property};
use crate::imran::lib::parse_value::{parse_obj, parse_string};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub properties: HashMap<String, Property>,
}

impl Config {
    pub fn from_json(default_config: DefaultConfig, config: Map<String, Value>) -> Config {
        let name = parse_string(&config, "name").unwrap().to_string();
        let mut properties: HashMap<String, Property> = HashMap::new();

        match config.get("properties") {
            Some(json_properties) => {
                for (key, val) in json_properties.as_object().unwrap().iter() {
                    properties.insert(
                        key.to_string(),
                        merge_default_properties(val.as_object().unwrap(), &default_config),
                    );
                }
            }
            None => {}
        }

        Config {
            name: name,
            properties: properties,
        }
    }
}

fn merge_default_properties(
    prop_config: &Map<String, Value>,
    default_config: &DefaultConfig,
) -> Property {
    let value_config = parse_obj(prop_config, "value").unwrap();
    let data_type = parse_string(value_config, "type").unwrap();

    let splits = data_type.split("default:");
    let types: Vec<&str> = splits.collect();
    let mut data_type = DataType::new();

    if types.len() > 1 {
        let default_value: &DataType = &default_config.data_types[types[1]];
        data_type.copy_from(default_value);
    } else {
        data_type.d_type = types[0].to_string();
    }

    Property {
        message_key: parse_string(prop_config, "message_key")
            .unwrap()
            .to_string(),
        value: data_type,
    }
}
