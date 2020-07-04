#![allow(dead_code)]

use crate::util::i18n;
use crate::util::server_status;

// Verb code
pub const GET: u8 = 0;
pub const POST: u8 = 1;
pub const PUT: u8 = 2;
pub const DELETE: u8 = 3;
pub const OPIONS: u8 = 4;

// Response code
// * The value field is text or a string containing name=value pairs seperated by semicolons. 
// * Individual handlers will need to know how to parse the values they use. They can use 
//   the to_json() function to convert name=value entries to a JSON string.
// * A colon in a path represents a parameter. The body of the request MUST supply the values
//   for these parameters.
// * For binary, text, and handlebars files:
//      * The requested file is in the body as JSON. The JSON has only a file member which contains only the file name. Example: {"file": "my.file"}" 
//      * The file entry in the config must contain one :file param.
//      * The file entry in the config maps the requested file to a file on the server.
//      * Example: 
//      * Given request body: {"file": "my.file"}
//      * Given config file param: ./required/path/to/requested/text/data/:file
//      * Server returns the file located at ./required/path/to/requested/text/data/my.file
// 0 = Text           "response": { "type": 0, "value": "Text goes here" }
// 1 = JSON           "response": { "type": 1, "value": "name=Server;version=1.0;"}
// 2 = Binary file    "response": { "type": 2, "file": "./required/path/to/binary/data/:file"}
// 3 = Text file      "response": { "type": 3, "file": "./required/path/to/requested/text/data/:file"}
// 4 = Handlebars     "response": { "type": 4, "value": "title=Handlebars template data;goes=here;" "file": "./required/path/to/handlebars/template/:file"}
// 5 = Computed       "response": { "type": 5 }
pub const TEXT: u8 = 0;
pub const JSON: u8 = 1;
pub const BINARY_FILE: u8 = 2;
pub const TEXT_FILE: u8 = 3;
pub const HANDLEBARS: u8 = 4;
pub const COMPUTED: u8 = 5;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ResponseInfo {
    pub code: u8,
    pub value: Option<String>,
    pub file: Option<String>,
}
impl ResponseInfo {
    pub fn new(code: u8, value: Option<String>, file: Option<String>) -> ResponseInfo {
        let response_info = ResponseInfo {
            code: code,
            value: value,
            file: file
        };
        response_info
    }
}

pub fn to_json(data: &str) -> Result<String, server_status::ServerStatus> {
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
// Used to describe a service.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ServiceEntry {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub response_info: ResponseInfo,
    pub authentication: Option<String>,
    pub authorization: Option<Authorization>,
    pub options: Option<String>,
}
impl ServiceEntry {
    pub fn new(
        id: u32,
        name: &str,
        description: &str,
        response_info: &ResponseInfo,
        authentication: &Option<String>,
        authorization: &Option<Authorization>,
        options: &Option<String>
    ) -> ServiceEntry {
        let service_entry = ServiceEntry {
            id: id,
            name: name.to_string(),
            description: description.to_string(),
            response_info: response_info.clone(),
            authentication: authentication.clone(),
            authorization: authorization.clone(),
            options: options.clone()
        };
        service_entry
    }
}

pub static DEFAULT_PORT: u32 = 8080;

// Used to store basic server info.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ServerInfo {
    pub port: Option<u32>,
    pub logging: Option<String>,
    pub locale: Option<String>,
    pub locale_path: Option<String>
}
impl ServerInfo {
    pub fn new(port: Option<u32>, logging: Option<String>, locale: Option<String>, locale_path: Option<String>) -> ServerInfo {
        let server_info = ServerInfo {
            port: port,
            logging: logging,
            locale: locale,
            locale_path: locale_path            
        };
        server_info
    }
    pub fn default() -> ServerInfo {
        let server_info = ServerInfo {
            port: Some(DEFAULT_PORT),
            logging: Some("WARN".to_string()),
            locale: Some(i18n::DEFAULT_LOCALE.to_string()),
            locale_path: Some(i18n::DEFAULT_PATH.to_string())
        };
        server_info
    }
    pub fn add_defaults(server_info: &ServerInfo) -> ServerInfo {
        let defaults = ServerInfo::default();
        let mut port = server_info.port;
        let mut logging = server_info.logging.clone();
        let mut locale = server_info.locale.clone();
        let mut locale_path = server_info.locale_path.clone();
        if None == port { port = defaults.port; }
        if None == logging { logging = defaults.logging; }
        if None == locale { locale = defaults.locale; }
        if None == locale_path { locale_path = defaults.locale_path; }
        ServerInfo::new(port, logging, locale, locale_path)
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
        let response_info = ResponseInfo::new(HANDLEBARS, Some("x=y;a=b;".to_string()), Some("file=a/b/c".to_string()));
        assert_eq!(response_info.code, HANDLEBARS);
        assert_eq!(response_info.value, Some("x=y;a=b;".to_string()));
        assert_eq!(response_info.file, Some("file=a/b/c".to_string()));

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
    fn test_to_json() {
        match to_json("test=my test;") {
            Ok(data) => assert_eq!(data, "{\"test\":\"my test\"}"),
            Err(_err) => assert_eq!(true, false),
        };
        match to_json("test=my test;test2=my test 2;") {
            Ok(data) => assert_eq!(data, "{\"test\":\"my test\",\"test2\":\"my test 2\"}"),
            Err(_err) => assert_eq!(true, false),
        };
        match to_json("test=my test;test2=my test 2;test3=my test 3;") {
            Ok(data) => assert_eq!(data, "{\"test\":\"my test\",\"test2\":\"my test 2\",\"test3\":\"my test 3\"}"),
            Err(_err) => assert_eq!(true, false),
        };
        match to_json("test=my test;test2=my test 2;test3=") {
            Ok(_data) => assert_eq!(true, false),
            Err(err) => assert_eq!(497, err.status),
        };
    }

    
    #[test]
    fn test_new_service_entry() {
        let response_info = ResponseInfo::new(HANDLEBARS, Some("x=y;a=b;".to_string()), Some("file=a/b/c".to_string()));
        let mut service_entry = ServiceEntry::new(
            2, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        assert_eq!(service_entry.id, 2);
        assert_eq!(service_entry.name, "name");
        assert_eq!(service_entry.description, "description");
        assert_eq!(service_entry.response_info, response_info);
        assert_eq!(service_entry.authentication, None);
        assert_eq!(service_entry.authorization, None);
        assert_eq!(service_entry.options, None);

        let groups = vec!["user".to_string(), "admin".to_string()];
        let authorization = Authorization::new( "strategy", &groups);
        service_entry = ServiceEntry::new(
            2, 
            "name", 
            "description", 
            &response_info, 
            &Some("Oauth".to_string()), 
            &Some(authorization.clone()), 
            &Some("option1=x".to_string()));
        assert_eq!(service_entry.id, 2);
        assert_eq!(service_entry.name, "name");
        assert_eq!(service_entry.description, "description");
        assert_eq!(service_entry.response_info, response_info);
        assert_eq!(service_entry.authentication, Some("Oauth".to_string()));
        assert_eq!(service_entry.authorization, Some(authorization));
        assert_eq!(service_entry.options, Some("option1=x".to_string()));
    }

    #[test]
    fn test_service_entry_from_json() {
        let mut data = r#"
            {
                "id": 42,
                "name": "name",
                "description": "description",
                "response_info": { "code": 3, "file": "/path/to/file"}
            }"#;

        let mut service_entry: ServiceEntry = serde_json::from_str(data).unwrap();
        assert_eq!(service_entry.id, 42);
        assert_eq!(service_entry.name, "name");
        assert_eq!(service_entry.description, "description");
        assert_eq!(service_entry.response_info.code, 3);
        assert_eq!(service_entry.response_info.file.unwrap(), "/path/to/file");
        assert_eq!(service_entry.authentication, None);
        assert_eq!(service_entry.authorization, None);
        assert_eq!(service_entry.options, None);

        data = r#"
            {
                "id": 43,
                "name": "name",
                "description": "description",
                "response_info": { "code": 3, "file": "/path/to/file"},
                "authentication": "0auth",
                "authorization": { "strategy": "strategy", "groups": ["admin", "user"] },
                "options": "option1=abc;"
            }"#;
        service_entry = serde_json::from_str(data).unwrap();
        assert_eq!(service_entry.id, 43);
        assert_eq!(service_entry.name, "name");
        assert_eq!(service_entry.description, "description");
        assert_eq!(service_entry.response_info.code, 3);
        assert_eq!(service_entry.response_info.file.unwrap(), "/path/to/file");
        assert_eq!(service_entry.authentication, Some("0auth".to_string()));
        let authorization: Authorization = service_entry.authorization.unwrap();
        assert_eq!(authorization.strategy, "strategy");
        assert_eq!(authorization.groups[0], "admin");
        assert_eq!(authorization.groups[1], "user");
        assert_eq!(service_entry.options, Some("option1=abc;".to_string()));

        data = r#"
            [
            {
                "id": 42,
                "name": "name",
                "description": "description",
                "response_info": { "code": 3, "file": "/path/to/file"}
            },
            {
                "id": 43,
                "name": "name",
                "description": "description",
                "response_info": { "code": 3, "file": "/path/to/file"},
                "authentication": "0auth",
                "authorization": { "strategy": "strategy", "groups": ["admin", "user"] },
                "options": "option1=abc;"
            }
        ]"#;
        let service_entries: Vec<ServiceEntry> = serde_json::from_str(data).unwrap();
        let service_entry1 = &service_entries[0];
        let service_entry2 = &service_entries[1];
        assert_eq!(service_entry1.id, 42);
        assert_eq!(service_entry1.name, "name");
        assert_eq!(service_entry1.description, "description");
        assert_eq!(service_entry1.response_info.code, 3);
        assert_eq!(service_entry1.response_info.file, Some("/path/to/file".to_string()));
        assert_eq!(service_entry1.authentication, None);
        assert_eq!(service_entry1.authorization, None);
        assert_eq!(service_entry1.options, None);

        assert_eq!(service_entry2.id, 43);
        assert_eq!(service_entry2.name, "name");
        assert_eq!(service_entry2.description, "description");
        assert_eq!(service_entry2.response_info.code, 3);
        assert_eq!(service_entry2.response_info.file, Some("/path/to/file".to_string()));
        assert_eq!(service_entry2.authentication, Some("0auth".to_string()));
        let authorization_ref = service_entry2.authorization.as_ref().unwrap();
        assert_eq!(authorization_ref.strategy, "strategy");
        assert_eq!(authorization_ref.groups[0], "admin");
        assert_eq!(authorization_ref.groups[1], "user");
        assert_eq!(service_entry2.options, Some("option1=abc;".to_string()));
    }

    #[test]
    fn test_new_server_info() {
        let server_info = ServerInfo::new( Some(100), Some("ALL".to_string()), Some(i18n::DEFAULT_LOCALE.to_string()), Some(i18n::DEFAULT_PATH.to_string()));
        assert_eq!(server_info.port, Some(100));
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
        assert_eq!(server_info.port, Some(DEFAULT_PORT));
        assert_eq!(server_info.logging, Some("WARN".to_string()));
        assert_eq!(server_info.locale, Some(i18n::DEFAULT_LOCALE.to_string()));
        assert_eq!(server_info.locale_path, Some(i18n::DEFAULT_PATH.to_string()));

        let json = serde_json::to_string(&server_info).unwrap();
        let server_info2: ServerInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(server_info, server_info2);
    }

    #[test]
    fn test_add_defaults_server_info() {
        let server_info_none = ServerInfo::new( None, None, None, None);
        let server_info = ServerInfo::add_defaults(&server_info_none);
        assert_eq!(server_info.port, Some(DEFAULT_PORT));
        assert_eq!(server_info.logging, Some("WARN".to_string()));
        assert_eq!(server_info.locale, Some(i18n::DEFAULT_LOCALE.to_string()));
        assert_eq!(server_info.locale_path, Some(i18n::DEFAULT_PATH.to_string()));
    }

    #[test]
    fn test_new_server_config() {
        let data = r#"{
            "server": {
                "port": 100,
                "logging": "ALL"
            },
            "services":
            [
            {
                "id": 42,
                "name": "name",
                "description": "description",
                "response_info": { "code": 3, "file": "/path/to/file"}
            },
            {
                "id": 43,
                "name": "name",
                "description": "description",
                "response_info": { "code": 3, "file": "/path/to/file"},
                "authentication": "0auth",
                "authorization": { "strategy": "strategy", "groups": ["admin", "user"] },
                "options": "option1=abc;"
            }
        ]}"#;
        let server_config: ServerConfig = serde_json::from_str(data).unwrap();
        let server = server_config.server;
        let services = server_config.services;

        assert_eq!(server.port, Some(100));
        assert_eq!(server.logging, Some("ALL".to_string()));

        let service_entry1 = &services[0];
        let service_entry2 = &services[1];
        assert_eq!(service_entry1.id, 42);
        assert_eq!(service_entry1.name, "name");
        assert_eq!(service_entry1.description, "description");
        assert_eq!(service_entry1.response_info.code, 3);
        assert_eq!(service_entry1.response_info.file, Some("/path/to/file".to_string()));
        assert_eq!(service_entry1.authentication, None);
        assert_eq!(service_entry1.authorization, None);
        assert_eq!(service_entry1.options, None);

        assert_eq!(service_entry2.id, 43);
        assert_eq!(service_entry2.name, "name");
        assert_eq!(service_entry2.description, "description");
        assert_eq!(service_entry2.response_info.code, 3);
        assert_eq!(service_entry2.response_info.file, Some("/path/to/file".to_string()));
        assert_eq!(service_entry2.authentication, Some("0auth".to_string()));
        let authorization_ref = service_entry2.authorization.as_ref().unwrap();
        assert_eq!(authorization_ref.strategy, "strategy");
        assert_eq!(authorization_ref.groups[0], "admin");
        assert_eq!(authorization_ref.groups[1], "user");
        assert_eq!(service_entry2.options, Some("option1=abc;".to_string()));
    }
}