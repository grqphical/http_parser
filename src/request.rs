use std::str::Utf8Error;
use crate::types::*;

/// A parsed HTTP Request
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub url: String,
    pub version: HttpVersion,
    pub method: HttpMethod,
    pub header: Header,
    pub contents: Vec<u8>
}

impl HttpRequest {
    /// Constructs an empty request
    pub fn new() -> Self {
        Self {
            url: String::new(),
            version: HttpVersion::Http1_1,
            method: HttpMethod::Get,
            header: Header::new(),
            contents: vec![]
        }
    }

    /// Creates a request object from an HTTP Request String
    /// 
    /// # Arguments
    /// `data` - The request to parse
    /// 
    /// # Examples
    /// 
    /// ```
    /// use http_parser::request::HttpRequest;
    /// 
    /// fn main() {
    ///     let data = String::from("GET / HTTP/1.1\nHost: www.example.com\n");
    ///     let request = HttpRequest::from_request(data);
    /// }
    /// ```
    pub fn from_request(data: String) -> Self {
        let mut lines: Vec<&str> = data.lines().collect();
        let http_info: &str = lines.remove(0);

        let http_info: Vec<&str> = http_info.split(" ").collect();
        let method = HttpMethod::try_from(http_info[0]).unwrap_or(HttpMethod::Get);
        let url = http_info[1];
        let version = HttpVersion::try_from(http_info[2]).unwrap_or(HttpVersion::Http1_1);

        let mut header: Header = Header::new();

        for line in &lines {
            if *line == "" {
                break;
            }
            let split = line.split(":").collect::<Vec<&str>>();
            let key = split[0];
            let value = split[1].trim_start();

            header.insert(key.to_string(), value.to_string());
        }

        let content_start_index: usize;
        match lines.iter().position(|&v| v == "") {
            Some(index) => content_start_index = index + 1,
            None => return HttpRequest {url: url.to_string(), version, method, header, contents: vec![]}
        }

        let contents_str: String = lines[content_start_index..].join("\n");
        let contents = contents_str.as_bytes().into_iter().map(|s| s.to_owned()).collect();

        return HttpRequest {url: url.to_string(), version, method, header, contents}
    }

    /// Returns the contents of the request as a string
    /// 
    /// # Example
    /// ```
    /// use http_parser::request::HttpRequest;
    /// 
    /// fn main() {
    ///     let data = String::from("GET / HTTP/1.1\nHost: www.example.com\n\nHello World!");
    ///     let request = HttpRequest::from_request(data);
    /// 
    ///     println!("{}", request.body().unwrap());
    /// # assert_eq!(request.body().unwrap(), String::from("Hello World!"))
    /// }
    /// ```
    pub fn body(&self) -> Result<String, Utf8Error> {
        match std::str::from_utf8(&self.contents) {
            Ok(value) => Ok(value.to_string()),
            Err(err) => Err(err)
        }
    }
}