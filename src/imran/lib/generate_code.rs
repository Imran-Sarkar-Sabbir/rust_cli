use crate::imran::lib::read_json::read_json_file;
use crate::imran::structs::config::Config;
use crate::imran::structs::default_config::DefaultConfig;

fn read_config() -> Config {
    let default_config_json = read_json_file("default.json".to_string());
    let config_json = read_json_file("config.json".to_string());
    let default_config = DefaultConfig::from_map(default_config_json);
    Config::from_json(default_config, config_json)
}

pub fn generate_code() {
    let config = read_config();
    print!("{:?}", config);
}
