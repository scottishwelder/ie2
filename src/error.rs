use std::fmt::{Display, Formatter, Result};
use std::io::ErrorKind;

type STR = &'static str;

#[derive(Debug)]
pub enum Error {
    NotFound(STR),
    PermissionDenied(STR),
    CommandFailed(STR),
    Unknown(STR),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => Error::NotFound(""),
            ErrorKind::PermissionDenied => Error::PermissionDenied(""),
            _ => Error::Unknown(""),
        }
    }
}
