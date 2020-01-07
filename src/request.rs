#![allow(dead_code)]
use std::collections::HashMap;

// Converts request data into a JSON object used by Handlers.
pub trait BodyParser {
    type ParseResult;
    fn name() -> &'static str;
    fn parse(buf: &mut [u8]) -> Self::ParseResult;
}

// The result of a handler processing a request.
pub struct EventResult<'a> {
    pub code: u8,
    pub message: &'a str,
}
impl<'a> EventResult<'a> {
    pub fn new(code: u8, message: &'a str) -> EventResult<'a> { EventResult { code: code, message: message }}
}

// Processes a JSON object created by a Parser and returns a Status.
pub trait Handler {
    fn name() -> &'static str;
    fn process<'a, T>(request: &T) -> EventResult<'a>;
}

pub fn get_route(data: &[u8]) -> String {
    let data_as_string = String::from_utf8(data.to_vec()).unwrap_or("".to_string());
    let mut lines = data_as_string.lines();
    let route = lines.next().unwrap_or("").to_string();
    route
}

pub fn get_headers(data: &[u8], headers: &mut HashMap<String, String>) { 
    let data_as_string = String::from_utf8(data.to_vec()).unwrap_or("".to_string());
    let mut lines = data_as_string.lines();
    let _route = lines.next();
    headers.clear();
    loop {
        let line = lines.next();
        if None == line { break; }
        let header = line.unwrap_or("").to_string();
        if "" == header { break; }
        let key_value: Vec<&str> = header.split(": ").collect();
        headers.insert(key_value[0].to_string(), key_value[1].to_string());
    }
}

pub fn get_body(data: &[u8], body: &mut Vec<u8>) { 
    let data_as_string = String::from_utf8(data.to_vec()).unwrap_or("".to_string());
    let mut lines = data_as_string.lines();
    let _route = lines.next();
    loop {
        let line = lines.next();
        if None == line { break; }
        let header = line.unwrap_or("").to_string();
        if "" == header { break; }
    }
    let b: String = lines.collect();
    let bytes = b.into_bytes();
    let iterator = bytes.iter();
    body.clear();
    iterator.for_each(|x| body.push(*x));
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::request::*;
    use std::collections::HashMap;

    #[test]
    fn test_request() {
        let mut req = r#"GET /hello.htm HTTP/1.1
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
Host: www.tutorialspoint.com
Accept-Language: en-us
Accept-Encoding: gzip, deflate
Connection: Keep-Alive

BODY"#;
let req2 = r#"GET /hello.htm HTTP/1.1

BODY"#;
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut route = get_route(req.as_bytes());
        assert_eq!(route, "GET /hello.htm HTTP/1.1");
        route = get_route("".as_bytes());
        assert_eq!(route, "");

        get_headers(req.as_bytes(), &mut headers);
        assert_eq!(headers.len(), 5);
        assert_eq!(headers.get("User-Agent").unwrap(), "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)");
        assert_eq!(headers.get("Host").unwrap(), "www.tutorialspoint.com");
        assert_eq!(headers.get("Accept-Language").unwrap(), "en-us");
        assert_eq!(headers.get("Accept-Encoding").unwrap(), "gzip, deflate");
        assert_eq!(headers.get("Connection").unwrap(), "Keep-Alive");
        get_headers(req2.as_bytes(), &mut headers);
        assert_eq!(headers.len(), 0);

        let mut body_vec: Vec<u8> = Vec::new();
        get_body(req.as_bytes(), &mut body_vec);
        let mut body = String::from_utf8(body_vec).unwrap();
        assert_eq!(body, "BODY");
        req = r#"GET /hello.htm HTTP/1.1
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
Host: www.tutorialspoint.com
Accept-Language: en-us
Accept-Encoding: gzip, deflate
Connection: Keep-Alive

"#;
        body_vec = Vec::new();
        get_body(req.as_bytes(), &mut body_vec);
        body = String::from_utf8(body_vec).unwrap();
        assert_eq!(body, "");    
    }

    #[test]
    fn test_new_event_result() {
        let event_result = EventResult::new(200, "OK");
        assert_eq!(event_result.code, 200);
        assert_eq!(event_result.message, "OK");
    }
}