use std::{collections::HashMap, fmt};

use crate::types;
use http;

/// Represents an HTTP response
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    version: types::HttpVersion,
    status_code: u16,
    status_reason: String,
    header: HashMap<String, String>,
    body: String,
}

impl Response {
    /// Create a new request. Not meant to be used externally it is only used in the library's tests
    pub fn new(version: types::HttpVersion, status_code: u16, status_reason: String, header: HashMap<String, String>, body: String) -> Self {
        return Self {
            version,
            status_code,
            status_reason,
            header,
            body
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\n", self.version, self.status_code, self.status_reason)?;
        
        for (key, value) in self.header.clone() {
            write!(f, "{}: {}\n", key, value)?;
        }

        write!(f, "\n{}", self.body)
    }
}

/// Used to create HTTP responses
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseBuilder {
    response: Response
}

impl ResponseBuilder {
    /// Creates a new builder
    pub fn new() -> ResponseBuilder {
        ResponseBuilder {
            response: Response { version: types::HttpVersion::Http1_1, status_code: 0, status_reason: String::new(), header: HashMap::new(), body: String::new() }
        }
    }
    /// Adds an HTTP version to the response. Either 1.1, 2.0 or 3.0
    /// 
    /// # Example
    /// ```
    /// use http_parser::response::ResponseBuilder;
    /// use http_parser::types::HttpVersion;
    /// 
    /// fn main() {
    ///     let response = ResponseBuilder::new().version(HttpVersion::Http1_1).build();
    /// }
    /// ```
    pub fn version(mut self, version: types::HttpVersion) -> ResponseBuilder {
        self.response.version = version;
        return self
    }

    /// Adds an HTTP status to the response. Returns a result that will Err if it was provided an invalid HTTP code (eg. outside the range 100-1000)
    /// 
    /// # Example
    /// ```
    /// use http_parser::response::ResponseBuilder;
    /// 
    /// fn main() {
    ///     let builder = ResponseBuilder::new().status(200).unwrap().build();
    ///     
    ///     // This would return an error
    ///     let builder = ResponseBuilder::new().status(1001).unwrap_err();
    /// }
    /// ```
    pub fn status(mut self, code: u16) -> Result<ResponseBuilder, String> {
        let status = http::StatusCode::from_u16(code);
        match status {
            Ok(s) => {
                self.response.status_code = code;
                self.response.status_reason = s.canonical_reason().unwrap_or_default().to_string();
                return Ok(self)
            },
            Err(_) => return Err(String::from("Invalid HTTP status code"))
        }
    }

    /// Adds a header to the response
    /// 
    /// # Example
    /// ```
    /// use http_parser::response::ResponseBuilder;
    /// use std::collections::HashMap;
    /// 
    /// fn main() {
    ///     let mut header: HashMap<String, String> = HashMap::new();
    ///     header.insert("Content-Type".to_string(), "*/*".to_string());
    /// 
    ///     let response = ResponseBuilder::new().header(header).build();
    /// }
    /// ```
    pub fn header(mut self, header: HashMap<String, String>) -> ResponseBuilder {
        self.response.header = header;
        return self
    }

    /// Adds a body to the response
    /// 
    /// # Example
    /// ```
    /// use http_parser::response::ResponseBuilder;
    /// 
    /// fn main() {
    ///     let response = ResponseBuilder::new().body(String::from("Hello World!")).build();
    /// }
    /// ```
    pub fn body(mut self, body: String) -> ResponseBuilder {
        self.response.body = body;
        return self
    }

    /// Returns the built response
    /// 
    /// # Example
    /// ```
    /// use http_parser::response::ResponseBuilder;
    /// use std::collections::HashMap;
    /// 
    /// fn main() {
    ///     let response = ResponseBuilder::new().body(String::from("Hello World!")).build();
    ///     
    ///     println!("{:?}", response);
    /// }
    /// ```
    pub fn build(self) -> Response {
        return self.response
    }
}