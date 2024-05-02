use crate::imran::structs::config::Config;
use crate::imran::structs::data_type::Type;
use crate::imran::structs::property::Property;

pub fn generate_input_fields(config: &Config) -> String {
    let mut input_fields: Vec<String> = vec![];
    for (name, property) in config.properties.iter() {
        let input_field: String = match property.value.d_type {
            Type::none => "".to_string(),
            Type::string => generate_text_input(name, property),
            Type::number => generate_number_input(name, property),
            Type::select => "".to_string(),
            Type::checkbox => "".to_string(),
            Type::radio => "".to_string(),
        };
        input_fields.push(input_field);
    }
    input_fields.join("")
}


fn generate_text_input(name: &String, property: &Property) -> String {
    const TEXT_TEMP: &str = r#"
         <Grid item xs={6}>
          <CustomTextInput[[required]]
            id='[[name]]'
            control={control}
            label={messages['common.[[name]]']}
            register={register}
            errorInstance={errors}
            isLoading={isLoading}
          />
        </Grid>"#;
    let result = TEXT_TEMP.replace("[[name]]", name);
    return if property.value.required {
        result.replace("[[required]]", "\n            required")
    } else {
        result.replace("[[required]]", "")
    };
}

fn generate_number_input(name: &String, property: &Property) -> String {
    const TEXT_TEMP: &str = r#"
         <Grid item xs={6}>
          <CustomTextInput[[required]]
            id='[[name]]'
            type={'number'}
            control={control}
            label={messages['common.[[name]]']}
            register={register}
            errorInstance={errors}
            isLoading={isLoading}
          />
        </Grid>"#;
    let result = TEXT_TEMP.replace("[[name]]", name);
    return if property.value.required {
        result.replace("[[required]]", "\n            required")
    } else {
        result.replace("[[required]]", "")
    };
}