use crate::imran::lib::read_json::read_json_file;
use crate::imran::structs::default_config::DefaultConfig;

pub fn generate_code() {
    let default_config_json = read_json_file("default.json".to_string());
    let config_json = read_json_file("config.json".to_string());

    DefaultConfig::from_map(default_config_json);

}