use std::fmt::{Formatter, Result as FmtResult, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallbackConfigurationReadError<R, F> {
    read_error: R,
    fallback_error: F,
}

impl<R, F> FallbackConfigurationReadError<R, F> {
    pub fn new(read_error: R, fallback_error: F) -> Self {
        Self {
            read_error: read_error,
            fallback_error: fallback_error,
        }
    }

    pub fn read_error(&self) -> &R {
        &self.read_error
    }

    pub fn fallback_error(&self) -> &F {
        &self.fallback_error
    }
}

impl<R: Display, F: Display> Display for FallbackConfigurationReadError<R, F> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,
               "Read: {}\nFallback: {}",
               self.read_error,
               self.fallback_error)
    }
}