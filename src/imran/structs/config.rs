use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::imran::lib::parse_value::{parse_bool, parse_number, parse_obj, parse_string};
use crate::imran::structs::name::Name;

use super::{data_type::DataType, default_config::DefaultConfig, property::Property};

#[derive(Debug)]
pub struct Config {
    pub name: Name,
    pub pattern: HashMap<String, String>,
    pub properties: HashMap<String, Property>,
}

impl Config {
    pub fn from_json(default_config: DefaultConfig, config: Map<String, Value>) -> Config {
        let name = parse_obj(&config, "name").unwrap();

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

        let mut pattern = default_config.pattern;
        match config.get("pattern") {
            Some(json_pattern) => {
                for (key, val) in json_pattern.as_object().unwrap().iter() {
                    pattern.insert(key.clone(), val.as_str().unwrap().to_string());
                }
            }
            None => {}
        }

        Config {
            name: Name::from_json(name),
            properties,
            pattern,
        }
    }
}

fn merge_default_properties(
    prop_config: &Map<String, Value>,
    default_config: &DefaultConfig,
) -> Property {
    let value_config = parse_obj(prop_config, "value").unwrap();
    let type_ops = parse_string(value_config, "type").unwrap();

    let splits = type_ops.split("default:");
    let types: Vec<&str> = splits.collect();
    let mut data_type = DataType::new();

    if types.len() > 1 {
        let default_value: &DataType = &default_config.data_types[types[1]];
        data_type.copy_from(default_value);
    } else {
        data_type.d_type = types[0].to_string();
    }

    match parse_bool(value_config, "required") {
        None => {}
        Some(res) => {
            data_type.required = res;
        }
    }

    let opt = parse_number(value_config, "min");
    match opt {
        None => {}
        Some(_) => {
            data_type.min = opt;
        }
    }

    let opt = parse_number(value_config, "max");
    match opt {
        None => {}
        Some(_) => {
            data_type.max = opt;
        }
    }

    let opt = parse_number(value_config, "row");
    match opt {
        None => {}
        Some(_) => {
            data_type.row = opt;
        }
    }

    let opt = parse_number(value_config, "col");
    match opt {
        None => {}
        Some(_) => {
            data_type.col = opt;
        }
    }

    Property {
        message_key: parse_string(prop_config, "message_key")
            .unwrap()
            .to_string(),
        value: data_type,
    }
}
