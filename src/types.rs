use std::{collections::HashMap, fmt};

use crate::errors::{HttpMethodError, HttpVersionError};

/// Represents all the current HTTP Versions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    Http1_1,
    Http2,
    Http3
}

impl TryFrom<String> for HttpVersion {
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "HTTP/1.1" => Ok(HttpVersion::Http1_1),
            "HTTP/2" => Ok(HttpVersion::Http2),
            "HTTP/3" => Ok(HttpVersion::Http3),
            _ => Err(HttpVersionError)
        }
    }

    type Error = HttpVersionError;
}

impl TryFrom<&str> for HttpVersion {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "HTTP/1.1" => Ok(HttpVersion::Http1_1),
            "HTTP/2" => Ok(HttpVersion::Http2),
            "HTTP/3" => Ok(HttpVersion::Http3),
            _ => Err(HttpVersionError)
        }
    }

    type Error = HttpVersionError;
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpVersion::Http1_1 => write!(f, "HTTP/1.1"),
            HttpVersion::Http2 => write!(f, "HTTP/2"),
            HttpVersion::Http3 => write!(f, "HTTP/3")
        }
    }
}

/// Represents an HTTP Method used
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Head
}

impl TryFrom<String> for HttpMethod {
    type Error = HttpMethodError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            "put" => Ok(HttpMethod::Put),
            "patch" => Ok(HttpMethod::Patch),
            "delete" => Ok(HttpMethod::Delete),
            "options" => Ok(HttpMethod::Options),
            "head" => Ok(HttpMethod::Head),
            _ => Err(HttpMethodError),
        }
    }
}

impl TryFrom<&str> for HttpMethod {
    type Error = HttpMethodError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            "put" => Ok(HttpMethod::Put),
            "patch" => Ok(HttpMethod::Patch),
            "delete" => Ok(HttpMethod::Delete),
            "options" => Ok(HttpMethod::Options),
            "head" => Ok(HttpMethod::Head),
            _ => Err(HttpMethodError),
        }
    }
}

pub type Header = HashMap<String, String>;