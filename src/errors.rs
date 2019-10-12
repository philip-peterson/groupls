use std::io::{Error as IoError, ErrorKind};
use std::boxed::{Box};
use std::fmt::{Display};

pub trait Error = Display;

pub fn missing_field_error<'a>(field_name: &'static str) -> Box<dyn Error> {
    return Box::new(
        IoError::new(ErrorKind::Other, format!("Invalid line (missing field: {})", field_name))
    );
}

pub fn invalid_system_state<'a>(field_name: &'static str) -> Box<dyn Error> {
    return Box::new(
        IoError::new(ErrorKind::Other, format!("Invalid system configuration file state (invalid {})", field_name))
    );
}

pub fn invalid_file_syntax<'a>(file_name: &'static str) -> Box<dyn Error> {
    return Box::new(
        IoError::new(ErrorKind::Other, format!("Invalid system configuration file state (invalid syntax) in {}", file_name))
    );
}