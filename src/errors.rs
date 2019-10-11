use std::io::{Error, ErrorKind};

pub fn missing_field_error(field_name: &'static str) -> Error {
    return Error::new(ErrorKind::Other, format!("Invalid line (missing field: {})", field_name));
}