// https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer
// https://github.com/rust-num/num-derive/blob/master/tests/trivial.rs

#![allow(dead_code)]

use crate::i18n;
use crate::server_status;

// Verb code
pub static GET: u8 = 0;
pub static POST: u8 = 1;
pub static PUT: u8 = 2;
pub static DELETE: u8 = 3;
pub static OPIONS: u8 = 4;

// Response code
// * The value field is text or a string containing name=value pairs seperated by semicolons. 
// * Individual handlers will need to know how to parse the values they use. They can use 
//   the parse_values() function to convert name=value entries to a JSON string.
// * A colon in a path represents a parameter. The body of the request MUST supply the values
//   for these parameters.
// 0 = Text           "response": { "type": 0, "value": "Text goes here"}
// 1 = JSON           "response": { "type": 1, "value": "name=Server;version=1.0;"}
// 2 = Binary file    "response": { "type": 2, "value": "template=./path/to/binary/data/file;"}
// 3 = Text file      "response": { "type": 3, "value": "template=./:path/:to/:text/:data/:file"}
// 4 = Handlebars     "response": { "type": 4, "value": "file=./required/path/to/handlebars/template;title=Handlebars template data;goes=here;"}
// 5 = Computed       "response": { "type": 5, "value": "hasBody=false;"}
pub static TEXT: u8 = 0;
pub static JSON: u8 = 1;
pub static BINARY_FILE: u8 = 2;
pub static TEXT_FILE: u8 = 3;
pub static HANDLEBARS: u8 = 4;
pub static COMPUTED: u8 = 5;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ResponseInfo {
    code: u8,
    value: String
}
impl ResponseInfo {
    pub fn new(code: u8, value: & str) -> ResponseInfo {
        let response_info = ResponseInfo {
            code: code,
            value: value.to_string()
        };
        response_info
    }
}

fn parse_values(data: &str) -> Result<String, server_status::ServerStatus> {
    let pairs = data.split(";").collect::<Vec<_>>();
    if 0 == pairs.len() { return Ok("".to_string()); }
    let mut pairs_iter = pairs.iter();

    let mut json = "{".to_owned();
    let mut count = 0;
    loop {
        match pairs_iter.next() {
            Some(pair) => {
                if 0 == pair.len() { break; }
                let mut key_value_iter = pair.split("=");
                match key_value_iter.next() {
                    Some(key) => {
                        match key_value_iter.next() {
                            Some(value) => {
                                if 0 == value.len() {
                                    let mut err: server_status::ServerStatus = server_status::INVALID_KEY_VALUE_PAIRS.clone();
                                    err.context = data.to_string();
                                    return Err(err);
                                }
                                if 0 < count { json.push_str(","); }
                                json.push_str("\""); json.push_str(key); json.push_str("\":");
                                json.push_str("\""); json.push_str(value); json.push_str("\"");
                                count += 1;
                            },
                            None => {
                                let mut err: server_status::ServerStatus = server_status::INVALID_KEY_VALUE_PAIRS.clone();
                                err.context = data.to_string();
                                return Err(err);
                            }
                        };
                    },
                    None => break
                };
            }
            None => break
        }
    }
    json.push_str("}");
    Ok(json)
}

// Service entry
// {
//   "id": unique id number,
//   "name": descriptive name,
//   "description": A shrt description,
//   "response": { "type": response_code, "value": response_value}
//   "authentication": "authentication_strategy_name",                  // Optional
//   "authorization": { "strategy": "local", "groups": [ "admin" ] },   // Optional
//   "options": "title=Handler specific data;goes=here;",               // Optional
//   "http": {                                                          // Optional
//      "verb": verb_code, 
//      "path": http path string, including parameters, if used. No query strings, schema, or authority,
//       "headers": [                                                   // Optional
//          { "name": "MY_HEADER1", "value": "MY_HEADER_VALUE1" },
//          { "name": "MY_HEADER2", "value": "MY_HEADER_VALUE2" },
//          { "name": "MY_HEADER3", "value": "MY_HEADER_VALUE3" }
//       ]
//   }
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Authorization {
    pub strategy: String,
    pub groups: Vec<String>
}
impl Authorization {
    pub fn new(strategy: &str, groups: &Vec<String>) -> Authorization {
        let authorization = Authorization {
            strategy: strategy.to_string(),
            groups: groups.clone()
        };
        authorization
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct HTTPHeader {
    pub name: String,
    pub value: String,
}
impl HTTPHeader {
    pub fn new(name: & str, value: & str) -> HTTPHeader {
        let header = HTTPHeader {
            name: name.to_string(),
            value: value.to_string()
        };
        header
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct HTTP {
    pub verb: u8,
    pub path: String,
    pub headers: Option<Vec<HTTPHeader>>
}
impl HTTP {
    pub fn new(verb: u8, path: & str, headers: &Option<Vec<HTTPHeader>>) -> HTTP {
        let http = HTTP {
            verb: verb,
            path: path.to_string(),
            headers: headers.clone()
        };
        http
    }
}
// Used to describe a service.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ServiceEntry {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub response: ResponseInfo,
    pub authentication: Option<String>,
    pub authorization: Option<Authorization>,
    pub options: Option<String>,
    pub http: Option<HTTP>
}
impl ServiceEntry {
    pub fn new(
        id: u32,
        name: &str,
        description: &str,
        response: &ResponseInfo,
        authentication: &Option<String>,
        authorization: &Option<Authorization>,
        options: &Option<String>,
        http: &Option<HTTP>
    ) -> ServiceEntry {
        let service_entry = ServiceEntry {
            id: id,
            name: name.to_string(),
            description: description.to_string(),
            response: response.clone(),
            authentication: authentication.clone(),
            authorization: authorization.clone(),
            options: options.clone(),
            http: http.clone()
        };
        service_entry
    }
}

// Used to store basic server info.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ServerInfo {
    pub port: Option<u32>,
    pub https_port: Option<u32>,
    pub logging: Option<String>,
    pub locale: Option<String>,
    pub locale_path: Option<String>
}
impl ServerInfo {
    pub fn new(port: Option<u32>, https_port: Option<u32>, logging: Option<String>, locale: Option<String>, locale_path: Option<String>) -> ServerInfo {
        let server_info = ServerInfo {
            port: port,
            https_port: https_port,
            logging: logging,
            locale: locale,
            locale_path: locale_path            
        };
        server_info
    }
    pub fn default() -> ServerInfo {
        let server_info = ServerInfo {
            port: Some(2),
            https_port: None,
            logging: Some("WARN".to_string()),
            locale: Some(i18n::DEFAULT_LOCALE.to_string()),
            locale_path: Some(i18n::DEFAULT_PATH.to_string())
        };
        server_info
    }
    pub fn add_defaults(server_info: &ServerInfo) -> ServerInfo {
        let defaults = ServerInfo::default();
        let mut port = server_info.port;
        let mut https_port = server_info.https_port;
        let mut logging = server_info.logging.clone();
        let mut locale = server_info.locale.clone();
        let mut locale_path = server_info.locale_path.clone();
        if None == port { port = defaults.port; }
        if None == https_port { https_port = defaults.https_port; }
        if None == logging { logging = defaults.logging; }
        if None == locale { locale = defaults.locale; }
        if None == locale_path { locale_path = defaults.locale_path; }
        ServerInfo::new(port, https_port, logging, locale, locale_path)
    }
}

// Used to configure the server.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ServerConfig {
    pub server: ServerInfo,
    pub services: Vec<ServiceEntry>
}
impl ServerConfig {
    pub fn new(server_info: &ServerInfo, service_entries: & Vec<ServiceEntry>) -> ServerConfig {
        let server_config = ServerConfig {
            server: server_info.clone(),
            services: service_entries.clone()
        };
        server_config
    }
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::config::*;
    use serde_json;

    #[test]
    fn test_verb_code() {
        let json = serde_json::to_string(&DELETE).unwrap();
        let verb: u8 = serde_json::from_str(&json).unwrap();
        assert_eq!(verb, DELETE);
    }

    #[test]
    fn test_response_code() {
        let json = serde_json::to_string(&HANDLEBARS).unwrap();
        let response: u8 = serde_json::from_str(&json).unwrap();
        assert_eq!(response, HANDLEBARS);
    }
    
    #[test]
    fn test_new_response_info() {
        let response_info = ResponseInfo::new(HANDLEBARS, "file=a/b/c;x=y;a=b;");
        assert_eq!(response_info.code, HANDLEBARS);
        assert_eq!(response_info.value, "file=a/b/c;x=y;a=b;");

        let json = serde_json::to_string(&response_info).unwrap();
        let response_info2: ResponseInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(response_info, response_info2);
    }

    #[test]
    fn test_authorization() {
        let groups = vec!["user".to_string(), "admin".to_string()];
        let authorization = Authorization::new( "strategy", &groups);
        assert_eq!(authorization.strategy, "strategy");
        assert_eq!(authorization.groups.len(), 2);

        let json = serde_json::to_string(&authorization).unwrap();
        let authorization2: Authorization = serde_json::from_str(&json).unwrap();
        assert_eq!(authorization, authorization2);
    }

    #[test]
    fn test_parse_values() {
        match parse_values("test=my test;") {
            Ok(data) => assert_eq!(data, "{\"test\":\"my test\"}"),
            Err(_err) => assert_eq!(true, false),
        };
        match parse_values("test=my test;test2=my test 2;") {
            Ok(data) => assert_eq!(data, "{\"test\":\"my test\",\"test2\":\"my test 2\"}"),
            Err(_err) => assert_eq!(true, false),
        };
        match parse_values("test=my test;test2=my test 2;test3=my test 3;") {
            Ok(data) => assert_eq!(data, "{\"test\":\"my test\",\"test2\":\"my test 2\",\"test3\":\"my test 3\"}"),
            Err(_err) => assert_eq!(true, false),
        };
        match parse_values("test=my test;test2=my test 2;test3=") {
            Ok(_data) => assert_eq!(true, false),
            Err(err) => assert_eq!(497, err.status),
        };
    }

    #[test]
    fn test_new_http_header() {
        let http_header = HTTPHeader::new( "Header", "value");
        assert_eq!(http_header.name, "Header");
        assert_eq!(http_header.value, "value");

        let json = serde_json::to_string(&http_header).unwrap();
        let http_header2: HTTPHeader = serde_json::from_str(&json).unwrap();
        assert_eq!(http_header, http_header2);
    }

    #[test]
    fn test_new_http() {
        let mut http = HTTP::new(0, "/ping", &None);
        assert_eq!(http.verb, GET);
        assert_eq!(http.path, "/ping");
        assert_eq!(http.headers, None);
        
        let http_header = HTTPHeader::new( "Header", "value");
        http = HTTP::new(0, "/ping", &Some(vec![http_header]));
        assert_eq!(http.verb, GET);
        assert_eq!(http.path, "/ping");
        let headers = http.headers.unwrap();
        assert_eq!(headers[0].name, "Header");
        assert_eq!(headers[0].value, "value");

        http = HTTP::new(0, "/ping", &Some(vec![HTTPHeader::new( "Header", "value")]));
        let json = serde_json::to_string(&http).unwrap();
        let http2: HTTP = serde_json::from_str(&json).unwrap();
        assert_eq!(http, http2);
    }
    
    #[test]
    fn test_new_service_entry() {
        let response_info = ResponseInfo::new(HANDLEBARS, "file=a/b/c;x=y;a=b;");
        let mut service_entry = ServiceEntry::new(
            2, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None,
            &None);
        assert_eq!(service_entry.id, 2);
        assert_eq!(service_entry.name, "name");
        assert_eq!(service_entry.description, "description");
        assert_eq!(service_entry.response, response_info);
        assert_eq!(service_entry.authentication, None);
        assert_eq!(service_entry.authorization, None);
        assert_eq!(service_entry.options, None);
        assert_eq!(service_entry.http, None);

        let groups = vec!["user".to_string(), "admin".to_string()];
        let authorization = Authorization::new( "strategy", &groups);
        let http_header = HTTPHeader::new( "Header", "value");
        let http = HTTP::new(0, "/ping", &Some(vec![http_header]));
        service_entry = ServiceEntry::new(
            2, 
            "name", 
            "description", 
            &response_info, 
            &Some("Oauth".to_string()), 
            &Some(authorization.clone()), 
            &Some("option1=x".to_string()),
            &Some(http.clone()));
            assert_eq!(service_entry.id, 2);
            assert_eq!(service_entry.name, "name");
            assert_eq!(service_entry.description, "description");
            assert_eq!(service_entry.response, response_info);
            assert_eq!(service_entry.authentication, Some("Oauth".to_string()));
            assert_eq!(service_entry.authorization, Some(authorization));
            assert_eq!(service_entry.options, Some("option1=x".to_string()));
            assert_eq!(service_entry.http, Some(http));
        }

    #[test]
    fn test_service_entry_from_json() {
        let mut data = r#"
            {
                "id": 42,
                "name": "name",
                "description": "description",
                "response": { "code": 3, "value": "/path/to/file"}
            }"#;

        let mut service_entry: ServiceEntry = serde_json::from_str(data).unwrap();
        assert_eq!(service_entry.id, 42);
        assert_eq!(service_entry.name, "name");
        assert_eq!(service_entry.description, "description");
        assert_eq!(service_entry.response.code, 3);
        assert_eq!(service_entry.response.value, "/path/to/file");
        assert_eq!(service_entry.authentication, None);
        assert_eq!(service_entry.authorization, None);
        assert_eq!(service_entry.options, None);
        assert_eq!(service_entry.http, None);

        data = r#"
            {
                "id": 43,
                "name": "name",
                "description": "description",
                "response": { "code": 3, "value": "/path/to/file"},
                "authentication": "0auth",
                "authorization": { "strategy": "strategy", "groups": ["admin", "user"] },
                "options": "option1=abc;",
                "http": {
                    "verb": 3, 
                    "path": "/ping",
                    "headers": [
                       { "name": "MY_HEADER1", "value": "MY_HEADER_VALUE1" },
                       { "name": "MY_HEADER2", "value": "MY_HEADER_VALUE2" },
                       { "name": "MY_HEADER3", "value": "MY_HEADER_VALUE3" }
                    ]
                }
            }"#;
        service_entry = serde_json::from_str(data).unwrap();
        assert_eq!(service_entry.id, 43);
        assert_eq!(service_entry.name, "name");
        assert_eq!(service_entry.description, "description");
        assert_eq!(service_entry.response.code, 3);
        assert_eq!(service_entry.response.value, "/path/to/file");
        assert_eq!(service_entry.authentication, Some("0auth".to_string()));
        let authorization: Authorization = service_entry.authorization.unwrap();
        assert_eq!(authorization.strategy, "strategy");
        assert_eq!(authorization.groups[0], "admin");
        assert_eq!(authorization.groups[1], "user");
        assert_eq!(service_entry.options, Some("option1=abc;".to_string()));
        let http: HTTP = service_entry.http.unwrap();
        let http_headers: Vec<HTTPHeader> = http.headers.unwrap();
        assert_eq!(http.verb, 3);
        assert_eq!(http.path, "/ping");
        assert_eq!(http_headers[0].name, "MY_HEADER1");
        assert_eq!(http_headers[0].value, "MY_HEADER_VALUE1");
        assert_eq!(http_headers[1].name, "MY_HEADER2");
        assert_eq!(http_headers[1].value, "MY_HEADER_VALUE2");
        assert_eq!(http_headers[2].name, "MY_HEADER3");
        assert_eq!(http_headers[2].value, "MY_HEADER_VALUE3");

        data = r#"
            [
            {
                "id": 42,
                "name": "name",
                "description": "description",
                "response": { "code": 3, "value": "/path/to/file"}
            },
            {
                "id": 43,
                "name": "name",
                "description": "description",
                "response": { "code": 3, "value": "/path/to/file"},
                "authentication": "0auth",
                "authorization": { "strategy": "strategy", "groups": ["admin", "user"] },
                "options": "option1=abc;",
                "http": {
                    "verb": 3, 
                    "path": "/ping",
                    "headers": [
                        { "name": "MY_HEADER1", "value": "MY_HEADER_VALUE1" },
                        { "name": "MY_HEADER2", "value": "MY_HEADER_VALUE2" },
                        { "name": "MY_HEADER3", "value": "MY_HEADER_VALUE3" }
                    ]
                }
            }
        ]"#;
        let service_entries: Vec<ServiceEntry> = serde_json::from_str(data).unwrap();
        let service_entry1 = &service_entries[0];
        let service_entry2 = &service_entries[1];
        assert_eq!(service_entry1.id, 42);
        assert_eq!(service_entry1.name, "name");
        assert_eq!(service_entry1.description, "description");
        assert_eq!(service_entry1.response.code, 3);
        assert_eq!(service_entry1.response.value, "/path/to/file");
        assert_eq!(service_entry1.authentication, None);
        assert_eq!(service_entry1.authorization, None);
        assert_eq!(service_entry1.options, None);
        assert_eq!(service_entry1.http, None);

        assert_eq!(service_entry2.id, 43);
        assert_eq!(service_entry2.name, "name");
        assert_eq!(service_entry2.description, "description");
        assert_eq!(service_entry2.response.code, 3);
        assert_eq!(service_entry2.response.value, "/path/to/file");
        assert_eq!(service_entry2.authentication, Some("0auth".to_string()));
        let authorization_ref = service_entry2.authorization.as_ref().unwrap();
        assert_eq!(authorization_ref.strategy, "strategy");
        assert_eq!(authorization_ref.groups[0], "admin");
        assert_eq!(authorization_ref.groups[1], "user");
        assert_eq!(service_entry2.options, Some("option1=abc;".to_string()));
        let http_ref = service_entry2.http.as_ref().unwrap();
        let http_headers_ref = http_ref.headers.as_ref().unwrap();
        assert_eq!(http_ref.verb, 3);
        assert_eq!(http_ref.path, "/ping");
        assert_eq!(http_headers_ref[0].name, "MY_HEADER1");
        assert_eq!(http_headers_ref[0].value, "MY_HEADER_VALUE1");
        assert_eq!(http_headers_ref[1].name, "MY_HEADER2");
        assert_eq!(http_headers_ref[1].value, "MY_HEADER_VALUE2");
        assert_eq!(http_headers_ref[2].name, "MY_HEADER3");
        assert_eq!(http_headers_ref[2].value, "MY_HEADER_VALUE3");
    }

    #[test]
    fn test_new_server_info() {
        let server_info = ServerInfo::new( Some(100), Some(101), Some("ALL".to_string()), Some(i18n::DEFAULT_LOCALE.to_string()), Some(i18n::DEFAULT_PATH.to_string()));
        assert_eq!(server_info.port, Some(100));
        assert_eq!(server_info.https_port, Some(101));
        assert_eq!(server_info.logging, Some("ALL".to_string()));
        assert_eq!(server_info.locale, Some(i18n::DEFAULT_LOCALE.to_string()));
        assert_eq!(server_info.locale_path, Some(i18n::DEFAULT_PATH.to_string()));

        let json = serde_json::to_string(&server_info).unwrap();
        let server_info2: ServerInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(server_info, server_info2);
    }

    #[test]
    fn test_default_server_info() {
        let server_info = ServerInfo::default();
        assert_eq!(server_info.port, Some(2));
        assert_eq!(server_info.https_port, None);
        assert_eq!(server_info.logging, Some("WARN".to_string()));
        assert_eq!(server_info.locale, Some(i18n::DEFAULT_LOCALE.to_string()));
        assert_eq!(server_info.locale_path, Some(i18n::DEFAULT_PATH.to_string()));

        let json = serde_json::to_string(&server_info).unwrap();
        let server_info2: ServerInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(server_info, server_info2);
    }

    #[test]
    fn test_add_defaults_server_info() {
        let server_info_none = ServerInfo::new( None, None, None, None, None);
        let server_info = ServerInfo::add_defaults(&server_info_none);
        assert_eq!(server_info.port, Some(2));
        assert_eq!(server_info.https_port, None);
        assert_eq!(server_info.logging, Some("WARN".to_string()));
        assert_eq!(server_info.locale, Some(i18n::DEFAULT_LOCALE.to_string()));
        assert_eq!(server_info.locale_path, Some(i18n::DEFAULT_PATH.to_string()));
    }

    #[test]
    fn test_new_server_config() {
        let data = r#"{
            "server": {
                "port": 100,
                "https_port": 101,
                "logging": "ALL"
            },
            "services":
            [
            {
                "id": 42,
                "name": "name",
                "description": "description",
                "response": { "code": 3, "value": "/path/to/file"}
            },
            {
                "id": 43,
                "name": "name",
                "description": "description",
                "response": { "code": 3, "value": "/path/to/file"},
                "authentication": "0auth",
                "authorization": { "strategy": "strategy", "groups": ["admin", "user"] },
                "options": "option1=abc;",
                "http": {
                    "verb": 3, 
                    "path": "/ping",
                    "headers": [
                        { "name": "MY_HEADER1", "value": "MY_HEADER_VALUE1" },
                        { "name": "MY_HEADER2", "value": "MY_HEADER_VALUE2" },
                        { "name": "MY_HEADER3", "value": "MY_HEADER_VALUE3" }
                    ]
                }
            }
        ]}"#;
        let server_config: ServerConfig = serde_json::from_str(data).unwrap();
        let server = server_config.server;
        let services = server_config.services;

        assert_eq!(server.port, Some(100));
        assert_eq!(server.https_port, Some(101));
        assert_eq!(server.logging, Some("ALL".to_string()));

        let service_entry1 = &services[0];
        let service_entry2 = &services[1];
        assert_eq!(service_entry1.id, 42);
        assert_eq!(service_entry1.name, "name");
        assert_eq!(service_entry1.description, "description");
        assert_eq!(service_entry1.response.code, 3);
        assert_eq!(service_entry1.response.value, "/path/to/file");
        assert_eq!(service_entry1.authentication, None);
        assert_eq!(service_entry1.authorization, None);
        assert_eq!(service_entry1.options, None);
        assert_eq!(service_entry1.http, None);

        assert_eq!(service_entry2.id, 43);
        assert_eq!(service_entry2.name, "name");
        assert_eq!(service_entry2.description, "description");
        assert_eq!(service_entry2.response.code, 3);
        assert_eq!(service_entry2.response.value, "/path/to/file");
        assert_eq!(service_entry2.authentication, Some("0auth".to_string()));
        let authorization_ref = service_entry2.authorization.as_ref().unwrap();
        assert_eq!(authorization_ref.strategy, "strategy");
        assert_eq!(authorization_ref.groups[0], "admin");
        assert_eq!(authorization_ref.groups[1], "user");
        assert_eq!(service_entry2.options, Some("option1=abc;".to_string()));
        let http_ref = service_entry2.http.as_ref().unwrap();
        let http_headers_ref = http_ref.headers.as_ref().unwrap();
        assert_eq!(http_ref.verb, 3);
        assert_eq!(http_ref.path, "/ping");
        assert_eq!(http_headers_ref[0].name, "MY_HEADER1");
        assert_eq!(http_headers_ref[0].value, "MY_HEADER_VALUE1");
        assert_eq!(http_headers_ref[1].name, "MY_HEADER2");
        assert_eq!(http_headers_ref[1].value, "MY_HEADER_VALUE2");
        assert_eq!(http_headers_ref[2].name, "MY_HEADER3");
        assert_eq!(http_headers_ref[2].value, "MY_HEADER_VALUE3");
    }
}