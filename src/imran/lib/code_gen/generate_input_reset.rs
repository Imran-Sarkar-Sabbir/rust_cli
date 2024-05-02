use crate::imran::structs::config::Config;
use crate::imran::structs::data_type::Type;

pub fn get_initial_value(config: &Config) -> String {
    let mut initial_values: Vec<String> = vec![];
    const TAB_SPACE: &str = "  ";
    for (name, property) in config.properties.iter() {
        let v: String = format!("{name}: {}", match &property.value.d_type {
            Type::string => "''",
            Type::number => "0",
            _ => "null",
        });
        initial_values.push(v);
    }
    initial_values.join(&format!(",\n{TAB_SPACE}"))
}

pub fn get_input_reset(config: &Config) -> String {
    let mut reset_data: Vec<String> = vec![];
    const TAB_SPACE: &str = "  ";
    for (name, property) in config.properties.iter() {
        let v: String = format!("{name}: itemData?.{name}");
        reset_data.push(v);
    }
    reset_data.join(&format!(",\n{TAB_SPACE}{TAB_SPACE}{TAB_SPACE}{TAB_SPACE}"))
}
