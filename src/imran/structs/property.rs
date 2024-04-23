use crate::imran::structs::data_type::DataType;

#[derive(Debug)]
pub struct Property {
    pub value: DataType,
    pub message_key: String,
}