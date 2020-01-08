// https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer
// https://github.com/rust-num/num-derive/blob/master/tests/trivial.rs

#![allow(dead_code)]

// HTTP Verb code
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
enum VerbCode {
    GET = 0,
    POST,
    PUT,
    DELETE,
    OPTIONS
}

// Response code
// Value is text or a string containing name=value pairs seperated by semicolons. 
// Individual handlers will need to know how to parse the values they use.
// 0 = Text           "response": { "type": 0, "value": "Text goes here"}
// 1 = JSON           "response": { "type": 1, "value": "name=Server;version=1.0;"}
// 2 = Binary file    "response": { "type": 2, "value": "template=./path/to/binary/data/file;"}
// 3 = Text file      "response": { "type": 3, "value": "template=./:path/:to/:text/:data/:file"}
// 4 = Handlebars     "response": { "type": 4, "value": "file=./path/to/handlebars/template;title=Handlebars template data;goes=here;"}
// 5 = Computed       "response": { "type": 5, "value": "hasBody=false;"}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
enum ResponseCode {
    Text = 0,
    JSON,
    BinaryFile,
    TextFile,
    Handlebars,
    Computed
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
struct ResponseInfo<'a> {
    code: ResponseCode,
    value: &'a str
}
impl<'a> ResponseInfo<'a> {
    pub fn new(code: ResponseCode, value: &'a str) -> ResponseInfo<'a> {
        let response_info = ResponseInfo {
            code: code,
            value: value
        };
        response_info
    }
}

// {
//   "id": unique id number,
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
//   },
//   "authorization": { "strategy": "local", "groups": [ "admin" ] },   // Optional
//   "body": { "param1": "value1", "param2": [ "value2" ] },            // Optional
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Authorization<'a> {
    pub strategy: &'a str,
    pub groups: Vec<String>
}
impl<'a> Authorization<'a> {
    pub fn new(strategy: &'a str, groups: &'a Vec<String>) -> Authorization<'a> {
        let authorization = Authorization {
            strategy: strategy,
            groups: groups.clone()
        };
        authorization
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct HTTPHeader<'a> {
    pub name: &'a str,
    pub value: &'a str,
}
impl<'a> HTTPHeader<'a> {
    pub fn new(name: &'a str, value: &'a str) -> HTTPHeader<'a> {
        let header = HTTPHeader {
            name: name,
            value: value
        };
        header
    }
}

// Used to store an entry from the config file.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigRequest<'a> {
    pub name: &'a str,          // Entry name.
    pub description: &'a str,   // Entry description.
    pub route: &'a str,         // Entry route.
    pub parser: &'a str,        // Parser name.
    pub handler: &'a str,       // Handler name.
}
impl<'a> ConfigRequest<'a> {
    pub fn new(name: &'a str, description: &'a str, route: &'a str, parser: &'a str, handler: &'a str) -> ConfigRequest<'a> {
        let config = ConfigRequest {
            name: name,
            description: description,
            route: route,
            parser: parser,
            handler: handler
        };
        config
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
        let json = serde_json::to_string(&VerbCode::DELETE).unwrap();
        let verb: VerbCode = serde_json::from_str(&json).unwrap();
        assert_eq!(verb, VerbCode::DELETE);
    }

    #[test]
    fn test_response_code() {
        let json = serde_json::to_string(&ResponseCode::Handlebars).unwrap();
        let response: ResponseCode = serde_json::from_str(&json).unwrap();
        assert_eq!(response, ResponseCode::Handlebars);
    }
    
    #[test]
    fn test_new_response_info() {
        let response_info = ResponseInfo::new( ResponseCode::Handlebars, "x=y;a=b;");
        assert_eq!(response_info.code, ResponseCode::Handlebars);
        assert_eq!(response_info.value, "x=y;a=b;");

        let json = serde_json::to_string(&response_info).unwrap();
        let response_info2: ResponseInfo<'_> = serde_json::from_str(&json).unwrap();
        assert_eq!(response_info, response_info2);
    }

    #[test]
    fn test_authorization() {
        let groups = vec!["user".to_string(), "admin".to_string()];
        let authorization = Authorization::new( "strategy", &groups);
        assert_eq!(authorization.strategy, "strategy");
        assert_eq!(authorization.groups.len(), 2);

        let json = serde_json::to_string(&authorization).unwrap();
        let authorization2: Authorization<'_> = serde_json::from_str(&json).unwrap();
        assert_eq!(authorization, authorization2);
    }

    #[test]
    fn test_new_http_header() {
        let http_header = HTTPHeader::new( "Header", "value");
        assert_eq!(http_header.name, "Header");
        assert_eq!(http_header.value, "value");

        let json = serde_json::to_string(&http_header).unwrap();
        let http_header2: HTTPHeader<'_> = serde_json::from_str(&json).unwrap();
        assert_eq!(http_header, http_header2);
    }
    
    #[test]
    fn test_new_config_request() {
        let config_request = ConfigRequest::new( "name", "description", "route", "parser", "handler");
        assert_eq!(config_request.name, "name");
        assert_eq!(config_request.description, "description");
        assert_eq!(config_request.route, "route");
        assert_eq!(config_request.parser, "parser");
        assert_eq!(config_request.handler, "handler");
    }

    #[test]
    fn test_config_request_from_json() {
        let data1 = r#"
            {
                "name": "name",
                "description": "description",
                "route": "route",
                "parser": "parser",
                "handler": "handler"
            }"#;
    
        let config_request1: ConfigRequest<'_> = serde_json::from_str(data1).unwrap();
        assert_eq!(config_request1.name, "name");
        assert_eq!(config_request1.description, "description");
        assert_eq!(config_request1.route, "route");
        assert_eq!(config_request1.parser, "parser");
        assert_eq!(config_request1.handler, "handler");

        let data_array1 = r#"
             [
                {
                    "name": "name1",
                    "description": "description1",
                    "route": "route1",
                    "parser": "parser1",
                    "handler": "handler1"
                },
                {
                    "name": "name2",
                    "description": "description2",
                    "route": "route2",
                    "parser": "parser2",
                    "handler": "handler2"
                },
                {
                    "name": "name3",
                    "description": "description3",
                    "route": "route3",
                    "parser": "parser3",
                    "handler": "handler3"
                }
             ]"#;
             
        let array: Vec<ConfigRequest<'_>> = serde_json::from_str(data_array1).unwrap();
        assert_eq!(array[0].name, "name1");
        assert_eq!(array[0].description, "description1");
        assert_eq!(array[0].route, "route1");
        assert_eq!(array[0].parser, "parser1");
        assert_eq!(array[0].handler, "handler1");
        assert_eq!(array[1].name, "name2");
        assert_eq!(array[1].description, "description2");
        assert_eq!(array[1].route, "route2");
        assert_eq!(array[1].parser, "parser2");
        assert_eq!(array[1].handler, "handler2");
        assert_eq!(array[2].name, "name3");
        assert_eq!(array[2].description, "description3");
        assert_eq!(array[2].route, "route3");
        assert_eq!(array[2].parser, "parser3");
        assert_eq!(array[2].handler, "handler3");
    }
}