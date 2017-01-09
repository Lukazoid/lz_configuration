use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq, Eq)]
pub enum CopyConfigurationError<R, W> {
    ReadError(R),
    WriteError(W),
}

impl<R: Display, W: Display> Display for CopyConfigurationError<R, W> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            CopyConfigurationError::ReadError(ref err) => write!(f, "Read Error {}", err),
            CopyConfigurationError::WriteError(ref err) => write!(f, "Write Error {}", err),
        }
    }
}

impl<R: Error, W: Error> Error for CopyConfigurationError<R, W> {
    fn description(&self) -> &str {
        match *self {
            CopyConfigurationError::ReadError(ref err) => err.description(),
            CopyConfigurationError::WriteError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CopyConfigurationError::ReadError(ref err) => Some(err),
            CopyConfigurationError::WriteError(ref err) => Some(err),
        }
    }
}
