use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub(crate) struct EphError {
    details: String
}

impl EphError {
    pub(crate) fn new(msg: &str) -> EphError {
        EphError { details: msg.to_string() }
    }
}

impl fmt::Display for EphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for EphError {
    fn description(&self) -> &str {
        &self.details
    }
}