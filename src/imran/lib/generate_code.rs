use crate::imran::lib::read_json::read_json_file;
use crate::imran::structs::config::Config;
use crate::imran::structs::default_config::DefaultConfig;

pub fn generate_code() {
    let default_config_json = read_json_file("default.json".to_string());
    let config_json = read_json_file("config.json".to_string());

    let default_config = DefaultConfig::from_map(default_config_json);
    let config = Config::from_json(default_config, config_json);
    print!("{:?}", config);
}
