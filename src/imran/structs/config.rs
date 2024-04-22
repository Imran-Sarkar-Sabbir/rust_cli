use std::iter::Map;

use crate::imran::structs::property::Property;

#[derive(Debug)]
pub struct Config {
    name: String,
    properties: Map<String, Property>,
}