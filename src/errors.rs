use std::fmt;

#[derive(Debug)]
pub struct HttpMethodError;

impl fmt::Display for HttpMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid HTTP method")
    }
}

impl std::error::Error for HttpMethodError {}

#[derive(Debug)]
pub struct HttpVersionError;

impl fmt::Display for HttpVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid HTTP version")
    }
}

impl std::error::Error for HttpVersionError {}