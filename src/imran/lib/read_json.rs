use std::fs;
use std::path::Path;
use serde_json::Value;

fn read_file(file_name: String) -> String {
    let path = Path::new("src").join(file_name);
    fs::read_to_string(path).expect("couldn't read file.")
}

pub fn read_json_file(file_name: String) -> serde_json::Map<String, Value> {
    let contents = read_file(file_name);
    let parsed: Value = serde_json::from_str(&contents).expect("error parsing json");
    parsed.as_object().unwrap().clone()
}