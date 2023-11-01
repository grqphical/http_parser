//! Easy to use HTTP Parser for Rust
//! 
//! Provides a simple, easy to use API for converting text requests to structs
pub mod errors;
pub mod request;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::request::HttpRequest;
    use crate::types::Header;

    #[test]
    fn no_body_request() {
        let request = "GET / HTTP/1.1
Host: www.example.com
User-Agent: Mozilla/5.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
Accept-Language: en-GB,en;q=0.5
Accept-Encoding: gzip, deflate, br
Connection: keep-alive";

        let req_obj = HttpRequest::from_request(request.into());
        assert_eq!(req_obj, HttpRequest { url: "/".to_string(), version: crate::types::HttpVersion::Http1_1, method: crate::types::HttpMethod::Get, 
            header: Header::from([
                ("Accept-Encoding".to_string(), "gzip, deflate, br".to_string()),
                ("Host".to_string(), "www.example.com".to_string()),
                ("Connection".to_string(), "keep-alive".to_string()),
                ("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string()),
                ("Accept-Language".to_string(), "en-GB,en;q=0.5".to_string()),
                ("User-Agent".to_string(), "Mozilla/5.0".to_string()),
        ]), contents: vec![]})
    }

    #[test]
    fn request_with_body() {
        let request = "GET / HTTP/1.1
Host: www.example.com
User-Agent: Mozilla/5.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
Accept-Language: en-GB,en;q=0.5
Accept-Encoding: gzip, deflate, br
Connection: keep-alive

Hello World!";
        let req_obj = HttpRequest::from_request(request.into());
        assert_eq!(req_obj, HttpRequest { url: "/".to_string(), version: crate::types::HttpVersion::Http1_1, method: crate::types::HttpMethod::Get, 
    header: Header::from([
        ("Accept-Encoding".to_string(), "gzip, deflate, br".to_string()),
        ("Host".to_string(), "www.example.com".to_string()),
        ("Connection".to_string(), "keep-alive".to_string()),
        ("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string()),
        ("Accept-Language".to_string(), "en-GB,en;q=0.5".to_string()),
        ("User-Agent".to_string(), "Mozilla/5.0".to_string()),
]), contents: vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]})
    }
}
