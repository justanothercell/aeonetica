use std::backtrace::{Backtrace};
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::panic::Location;
use std::process::exit;
use crate::{log_err, log_raw};

pub struct AError {
    et: AET,
    pub additional_info: Vec<String>,
    location: Location<'static>,
    trace: Backtrace,
}

impl AError {
    #[track_caller]
    pub fn new(et: AET) -> Self {
        Self {
            et,
            additional_info: vec![],
            location: *std::panic::Location::caller(),
            trace: Backtrace::force_capture()
        }
    }
    pub fn log_exit(&self) {
        log_err!("{self}");
        log_raw!("{}", self.trace);
        exit(1)
    }
}

pub enum AET {
    ValueError(String),
    DataError(String),
    IOError(String),
    NetworkError(String),
    ModError(String),
    ModConflict(String)
}

impl Display for AError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}\nlocation: {}", match &self.et {
            AET::ValueError(e) => format!("ValueError: {e}"),
            AET::DataError(e) => format!("DataError: {e}"),
            AET::IOError(e) => format!("IOError: {e}"),
            AET::NetworkError(e) => format!("NetworkError: {e}"),
            AET::ModError(e) => format!("IOError: {e}"),
            AET::ModConflict(e) => format!("IOError: {e}"),
        }, if self.additional_info.len() > 0 {
            format!("\n => {}", self.additional_info.join("\n => "))
        } else { String::new() }, self.location)
    }
}

impl From<std::io::Error> for AError {
    #[track_caller]
    fn from(value: Error) -> Self {
        AError::new(AET::IOError(value.to_string()))
    }
}

impl From<nanoserde::DeRonErr> for AError {
    #[track_caller]
    fn from(value: nanoserde::DeRonErr) -> Self {
        AError::new(AET::DataError(value.to_string()))
    }
}