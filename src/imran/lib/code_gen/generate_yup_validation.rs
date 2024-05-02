use crate::imran::structs::config::Config;
use crate::imran::structs::data_type::Type;

pub fn get_validation(config: &Config) -> String {
    let mut validations: Vec<String> = vec![];
    const TAB_SPACE: &str = "  ";
    for (name, property) in config.properties.iter() {
        let mut v: Vec<String> = vec![format!("{name}: yup")];

        match property.value.d_type {
            Type::none => {}
            Type::string => {
                v.push(".string()".to_string());
            }
            Type::number => {}
            Type::select => {}
            Type::checkbox => {}
            Type::radio => {}
        }

        if (property.value.required) {
            v.push(".required()".to_string());
        }
        v.push(format!(".label(messages['{}'] as string)", property.message_key));
        validations.push(v.join(&format!("\n{TAB_SPACE}{TAB_SPACE}{TAB_SPACE}{TAB_SPACE}")));
    }
    validations.join(&format!(",\n{TAB_SPACE}{TAB_SPACE}{TAB_SPACE}"))
}
