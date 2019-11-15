use std::fmt::Display;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::string::String;

pub trait Error = Display;

pub fn internal_error<'a>(message: String) -> Box<dyn Error> {
    return Box::new(IoError::new(
        IoErrorKind::Other,
        format!("Internal error ({})", message),
    ));
}

pub fn usage_error<'a>(message: String) -> Box<dyn Error> {
    return Box::new(IoError::new(IoErrorKind::Other, format!("{}", message)));
}

pub fn missing_field_error<'a>(field_name: &'static str) -> Box<dyn Error> {
    return Box::new(IoError::new(
        IoErrorKind::Other,
        format!("Invalid line (missing field: {})", field_name),
    ));
}

pub fn invalid_system_state<'a>(field_name: &'static str) -> Box<dyn Error> {
    return Box::new(IoError::new(
        IoErrorKind::Other,
        format!(
            "Invalid system configuration file state (invalid {})",
            field_name
        ),
    ));
}
