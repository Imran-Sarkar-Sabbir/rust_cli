use crate::imran::structs::data_type::DataType;

#[derive(Debug)]
pub struct Property {
    value: DataType,
    message_key: String,
}