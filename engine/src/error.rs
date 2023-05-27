use std::backtrace::{Backtrace};
use std::fmt::{Display, Formatter};
use std::panic::Location;
use std::process::exit;
use crate::log;
use colored::Colorize;

#[derive(Default, Debug)]
pub enum Fatality {
    WARN,
    #[default]
    DEFAULT,
    FATAL
}

impl Fatality {
    fn color(&self) -> colored::Color {
        match self {
            Self::WARN => colored::Color::BrightYellow,
            Self::DEFAULT => colored::Color::Red,
            Self::FATAL => colored::Color::BrightRed
        }
    }

    fn str(&self) -> &'static str {
        match self {
            Self::WARN => "warn",
            Self::DEFAULT => "default",
            Self::FATAL => "fatal"
        }
    }
}

pub trait ErrorValue: std::fmt::Debug + Display {}

pub trait IntoError {
    fn into_error(self) -> Box<Error>;
}

#[derive(Debug)]
pub struct Error {
    value: Box<dyn ErrorValue>,
    fatality: Fatality,
    location: Location<'static>,
    trace: Option<Backtrace>,
    additional: Vec<String>
}

impl Error {
    #[track_caller]
    pub fn new(value: impl ErrorValue + 'static, fatality: Fatality, trace: bool) -> Box<Self> {
        Box::new(Self {
            value: Box::new(value),
            fatality,
            location: *std::panic::Location::caller(),
            trace: trace.then(|| Backtrace::force_capture()),
            additional: vec![]
        })
    }

    #[track_caller]
    pub fn add_info(&mut self, info: impl ToString) {
        self.additional.push(info.to_string())
    }

    #[track_caller]
    pub fn value(&self) -> &Box<dyn ErrorValue> {
        &self.value
    }

    #[track_caller]
    pub fn fatality(&self) -> &Fatality {
        &self.fatality
    }

    #[track_caller]
    pub fn log(&self) {
        log!("{}", self.to_string())
    }

    #[track_caller]
    pub fn log_exit(&self) -> ! {
        self.log();
        exit(1)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = self.fatality.color();

        write!(f, "{}", self.location.to_string().color(color))?;
        write!(f, "{}", ": ".color(color))?;
        write!(f, "{}", self.fatality.str().color(color))?;
        write!(f, "{}", ": ".color(color))?;
        write!(f, "{}", self.value.to_string().color(color))?;

        if let Some(trace) = &self.trace {
            write!(f, "\nin: {}", trace)?
        }

        Ok(())
    }
}

impl<T: IntoError> From<T> for Box<Error> {
    fn from(value: T) -> Self {
        value.into_error()
    }
}

pub type ErrorResult<T> = Result<T, Box<Error>>;

pub mod builtin {
    use super::*;

    macro_rules! impl_error_value {
        ($name:ident) => {
            impl ErrorValue for $name {}
            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}: {}", stringify!($name), self.0)
                }
            }
        };
    }

    #[derive(Debug)]
    pub struct ValueError(pub String);
    impl_error_value!{ValueError}

    #[derive(Debug)]
    pub struct DataError(pub String);
    impl_error_value!{DataError}
    
    #[derive(Debug)]
    pub struct IOError(pub String);
    impl_error_value!{IOError}

    #[derive(Debug)]
    pub struct NetworkError(pub String);
    impl_error_value!{NetworkError}
    
    #[derive(Debug)]
    pub struct ModError(pub String);
    impl_error_value!{ModError}
    
    #[derive(Debug)]
    pub struct ModConflict(pub String);
    impl_error_value!{ModConflict}

    impl IntoError for nanoserde::DeRonErr {
        fn into_error(self) -> Box<Error> {
            Error::new(DataError(self.to_string()), Fatality::DEFAULT, true)
        }
    }

    impl IntoError for std::io::Error {
        fn into_error(self) -> Box<Error> {
            Error::new(IOError(self.to_string()), Fatality::DEFAULT, true)
        }
    }
}

pub trait ExpectLog {
    type Inner;
    fn expect_log(self) -> Self::Inner;
}

impl<T> ExpectLog for ErrorResult<T> {
    type Inner = T;
    fn expect_log(self) -> Self::Inner {
        match self {
            Err(err) => err.log_exit(),
            Ok(val) => val
        }
    }
}