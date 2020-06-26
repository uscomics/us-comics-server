use bytes::BytesMut;
use chrono::prelude::*;
use console::style;
use futures::SinkExt;
use http;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed};

use crate::config;
use crate::http_codec;
use crate::preprocessing::computed_preprocessor::computed_preprocessor;
use crate::preprocessing::file_preprocessor::file_preprocessor;
use crate::preprocessing::json_preprocessor::json_preprocessor;
use crate::preprocessing::preprocessing_response::PreprocessingResponse;
use crate::preprocessing::text_preprocessor::text_preprocessor;
use crate::processing::binary_file_processor::binary_file_processor;
// use crate::processing::computed_processor::computed_processor;
use crate::processing::json_processor::json_processor;
use crate::processing::processing_response::ProcessingResponse;
use crate::processing::text_file_processor::text_file_processor;
use crate::processing::text_processor::text_processor;
use crate::util::i18n;
use crate::util::log;
use crate::util::server_status;
use crate::util::strings;
use crate::util::url;
use crate::{structured_debug, structured_info, structured_error, structured_log, style_text};

pub static DEFAULT_PATH: &'static str = "/index/:id";

pub struct Server {
    config: config::ServerConfig,
    i18n: i18n::I18n,
    log: log::Log
}
lazy_static! {
    static ref CONFIG: Server = Server::init("./config/config.json").unwrap();
}

impl Server {
    pub fn new(config: config::ServerConfig, i18n: i18n::I18n, log: log::Log) -> Server {
        Server{
            config: config,
            i18n: i18n,
            log: log
        }
    }

    pub fn init(config: &str) -> std::io::Result<Server> {
        let mut file = File::open(config)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let server_config: config::ServerConfig = serde_json::from_str(contents.as_str())?;
        let server_info_with_defaults = config::ServerInfo::add_defaults(&server_config.server);
        let server_config_with_defaults = config::ServerConfig::new(&server_info_with_defaults, &server_config.services);

        let i18n = i18n::I18n::new(server_info_with_defaults.locale.clone().unwrap().as_str(), server_info_with_defaults.locale_path.clone().unwrap().as_str());
        let log_level = log::Log::get_level(server_info_with_defaults.logging.clone().unwrap().as_str());
        let log = log::Log::new(log_level, &i18n);
        structured_debug!( log, "{}", config);
        structured_debug!( log, "{}", serde_json::to_string(&server_info_with_defaults)?);
        structured_debug!( log, "{}", serde_json::to_string(&server_config.services)?);

        Ok(Server::new(server_config_with_defaults, i18n, log))
    }

    pub async fn start() -> Result<(), Box<dyn Error>> {
        // Parse the arguments, bind the TCP socket we'll be listening to, spin up
        // our worker threads, and start shipping sockets to those worker threads.
        let addr = Server::get_address(&CONFIG);
        let mut tokio_server = TcpListener::bind(&addr).await?;
        let mut incoming = tokio_server.incoming();
        let listening_message = CONFIG.i18n.get(strings::LISTENING_ON_PORT);
        structured_info!( CONFIG.log, "{} {}", listening_message, addr);

        while let Some(Ok(stream)) = incoming.next().await {
            tokio::spawn(async move {
                if let Err(e) = Server::read(stream).await {
                    let error_message = CONFIG.i18n.get(strings::REQUEST_NOT_READ);
                    structured_error!( CONFIG.log, "{} {:?}", error_message, e);
                }
            });
        }
        Ok(())
    }

    async fn read(stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut transport = Framed::new(stream, http_codec::Http);

        while let Some(request) = transport.next().await {
            match request {
                Ok(request) => {
                    let response = Server::respond(request);
                    transport.send(response).await?;
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    fn respond(req: http::request::Request<BytesMut>) -> http::response::Response<BytesMut> {
        let req_body = req.body();
        let service_entry = match Server::get_requested_service(req.uri().path(), &CONFIG) {
            Ok(se) => { se },
            Err(e) => { return e; }
        };
        let preprocessing_response = match Server::preprocess(&service_entry, &req_body) {
            Ok(pr) => { pr },
            Err(e) => { return e; }
        };
        let processing_response = match Server::process(&preprocessing_response) {
            Ok(pr) => { pr },
            Err(e) => { return e; }
        };
        match Server::build_response(&processing_response) {
            Ok(r) => return r,
            Err(e) => return e
        }
    }

    fn get_address(server: &Server) -> String {
        let port = match server.config.server.port {
            Some(port) => port,
            None => {
                let mut calculated_port = config::DEFAULT_PORT;
                for (key, value) in env::vars() {
                    if key == ("US_COMICS_PORT") { calculated_port = value.parse::<u32>().unwrap_or(config::DEFAULT_PORT); }
                }
                calculated_port
            }
        };
        format!("{}{}", "127.0.0.1:", port)
    }

    fn build_error_response(status: &server_status::ServerStatus, context: &str) -> http::response::Response<BytesMut> {
        let mut status_clone: server_status::ServerStatus = status.clone();
        status_clone.context = context.to_string();
        structured_error!( CONFIG.log, "{:?}", status_clone);
        let mut body = BytesMut::new();
        body.extend_from_slice(status.name.as_bytes());
        http::response::Response::builder().status(status.status).body(body).unwrap()
    }

    fn build_response(processing_response: &ProcessingResponse) -> Result<http::response::Response<BytesMut>, http::response::Response<BytesMut>> {
        let mut response = http::response::Response::builder()
            .status(processing_response.status.status.clone())
            .header("Content-Type", processing_response.mime.mime_type.clone());
        let mut headers = response.headers_mut().unwrap();
        processing_response.copy_to_headers(&mut headers);
        if !headers.contains_key("X-US-COMICS-FILE") { 
            let body = if None != processing_response.body { processing_response.body.clone().unwrap() } else { BytesMut::new() };
            return Ok(response.body(body).unwrap()); 
        }
        let value_bytes = headers.get("X-US-COMICS-FILE").unwrap().as_bytes();
        let value = std::str::from_utf8(value_bytes).unwrap();
        let body = Server::get_file_bytes(&String::from(value))?;
        Ok(response.body(body).unwrap())
    }

    fn get_requested_service(path: &str, server: &Server) -> Result<config::ServiceEntry, http::response::Response<BytesMut>> {
        if "/favicon.ico" == path { return Err(Server::build_error_response(&server_status::INVALID_SERVICE, path)); }
        if !url::matches(path, DEFAULT_PATH) {
            return Err(Server::build_error_response(&server_status::INVALID_SERVICE, path)); 
        }
        let mut params = HashMap::new();
        url::get_params(path, DEFAULT_PATH, &mut params);
        if !params.contains_key("id") { return Err(Server::build_error_response(&server_status::INVALID_SERVICE, path)); }
        let id = match params["id"].parse::<usize>() {
            Ok(i) => i,
            Err(e) => { return Err(Server::build_error_response(&server_status::INVALID_SERVICE, format!("{:?}", e).as_str())); }
        };
        if server.config.services.len() <= id { return Err(Server::build_error_response(&server_status::INVALID_SERVICE, format!("{}", id).as_str())); }

        Ok(CONFIG.config.services[id].clone())
    }

    fn preprocess(service_entry: &config::ServiceEntry, req_body: &BytesMut) -> Result<PreprocessingResponse, http::response::Response<BytesMut>> {
        let preprocess = match service_entry.response_info.code {
            config::TEXT => text_preprocessor(&service_entry, req_body),
            config::JSON => json_preprocessor(&service_entry, req_body),
            config::BINARY_FILE => file_preprocessor(&service_entry, req_body),
            config::TEXT_FILE => file_preprocessor(&service_entry, req_body),
            config::HANDLEBARS => file_preprocessor(&service_entry, req_body),
            config::COMPUTED => computed_preprocessor(&service_entry, req_body),
            _ => {
                return Err(Server::build_error_response(&server_status::INVALID_RESPONSE_CODE, ""));
            }
        };
        let preprocessing_reponse = match preprocess {
            Ok(pr) => pr,
            Err(err) => { return Err(Server::build_error_response(&err, "")); }
        };
        if server_status::ServerStatus::is_error(&preprocessing_reponse.status) {
            return Err(Server::build_error_response(&server_status::INVALID_SERVICE, ""));
        }
        Ok(preprocessing_reponse)
    }

    fn process(preprocessing_response: &PreprocessingResponse) -> Result<ProcessingResponse, http::response::Response<BytesMut>> {
        let process = match preprocessing_response.response_info.code {
            config::TEXT => text_processor(&preprocessing_response),
            config::JSON => json_processor(&preprocessing_response),
            config::BINARY_FILE => binary_file_processor(&preprocessing_response),
            config::TEXT_FILE => text_file_processor(&preprocessing_response),
            // config::HANDLEBARS => file_processor(&preprocessing_response),
            // config::COMPUTED => computed_processor(&preprocessing_response),
            _ => {
                return Err(Server::build_error_response(&server_status::INVALID_RESPONSE_CODE, ""));
            }
        };
        let processing_reponse = match process {
            Ok(pr) => pr,
            Err(err) => { return Err(Server::build_error_response(&err, "")); }
        };
        if server_status::ServerStatus::is_error(&processing_reponse.status) {
            return Err(Server::build_error_response(&server_status::INVALID_SERVICE, ""));
        }
        Ok(processing_reponse)
    }

    fn get_file_bytes(file_name: &String) -> Result<BytesMut, http::response::Response<BytesMut>> {
        let mut file = match std::fs::File::open(&file_name){
            Ok(f) => f,
            Err(_e) => return Err(Server::build_error_response(&server_status::NOT_FOUND, ""))
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(_f) => { 
                let mut data2 = BytesMut::with_capacity(0);
                data2.extend_from_slice(&data);
                return Ok(data2) 
            },
            Err(_e) => return Err(Server::build_error_response(&server_status::INTERNAL_SERVER_ERROR, ""))
        }
    }
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::server::*;
    use crate::util::mime;
    use bytes::{BytesMut, BufMut};

    fn build_server() -> Server{
        let mut file = File::open("./config/config.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let server_config: config::ServerConfig = serde_json::from_str(contents.as_str()).unwrap();
        let server_info_with_defaults = config::ServerInfo::add_defaults(&server_config.server);
        let server_config_with_defaults = config::ServerConfig::new(&server_info_with_defaults, &server_config.services);
        let i18n = i18n::I18n::new(server_info_with_defaults.locale.clone().unwrap().as_str(), server_info_with_defaults.locale_path.clone().unwrap().as_str());
        let log_level = log::Log::get_level(server_info_with_defaults.logging.clone().unwrap().as_str());
        let log = log::Log::new(log_level, &i18n);
        let server = Server::new(server_config_with_defaults, i18n, log);
        return server;
    }

    fn build_server_from(contents: &str) -> Server{
        let server_config: config::ServerConfig = serde_json::from_str(contents).unwrap();
        let server_info_with_defaults = config::ServerInfo::add_defaults(&server_config.server);
        let server_config_with_defaults = config::ServerConfig::new(&server_info_with_defaults, &server_config.services);
        let i18n = i18n::I18n::new(server_info_with_defaults.locale.clone().unwrap().as_str(), server_info_with_defaults.locale_path.clone().unwrap().as_str());
        let log_level = log::Log::get_level(server_info_with_defaults.logging.clone().unwrap().as_str());
        let log = log::Log::new(log_level, &i18n);
        let server = Server::new(server_config_with_defaults, i18n, log);
        return server;
    }

    #[test]
    fn test_get_address() {
        let mut server = build_server();
        let mut address = Server::get_address(&server);
        assert_eq!(address, format!("{}{}", "127.0.0.1:", server.config.server.port.unwrap()));

        let config = r#"
        {
            "server": {
                "logging": "DEBUG"
            },
            "services": [
              {
                "id": 0,
                "name": "ping",
                "description": "Pings the server",
                "response_info": { "code": 1, "value": "name=U.S. Comics Server;version=0.0.1"}
              }
            ]
          }
        "#;
        server = build_server_from(config);
        address = Server::get_address(&server);
        assert_eq!(address, format!("{}{}", "127.0.0.1:", config::DEFAULT_PORT));
    }

    #[test]
    fn test_build_error_response() {
        let error = Server::build_error_response(&server_status::INTERNAL_SERVER_ERROR, "");
        assert_eq!(error.status(), server_status::INTERNAL_SERVER_ERROR.status);
        assert_eq!(*error.body(), server_status::INTERNAL_SERVER_ERROR.name);
    }

    #[test]
    fn test_get_requested_service() {
        let server = build_server();
        match Server::get_requested_service("/index/0", &server) {
            Ok(service) => assert_eq!(service.id, 0),
            Err(_e) => assert_eq!(true, false)
        }
        match Server::get_requested_service("/index/999", &server) {
            Ok(service) => assert_eq!(true, false),
            Err(e) => assert_eq!(e.status(), server_status::INVALID_SERVICE.status)
        }
        match Server::get_requested_service("/index", &server) {
            Ok(service) => assert_eq!(true, false),
            Err(e) => assert_eq!(e.status(), server_status::INVALID_SERVICE.status)
        }
        match Server::get_requested_service("/JUNK", &server) {
            Ok(service) => assert_eq!(true, false),
            Err(e) => assert_eq!(e.status(), server_status::INVALID_SERVICE.status)
        }
        match Server::get_requested_service("/JUNK/0", &server) {
            Ok(service) => assert_eq!(true, false),
            Err(e) => assert_eq!(e.status(), server_status::INVALID_SERVICE.status)
        }
    }

    #[test]
    fn test_preprocess() {
        let mut response_info = config::ResponseInfo::new(config::TEXT, Some("Text goes here".to_string()), None);
        let mut service_entry = config::ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None
        );
        let mut response = Server::preprocess(&service_entry, &BytesMut::new());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::TEXT);
                assert_eq!(r.value, Some("Text goes here".to_string()));
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        }
        
        response_info = config::ResponseInfo::new(config::JSON, Some("name=Server;version=1.0;".to_string()), None);
        service_entry = config::ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        let mut response = Server::preprocess(&service_entry, &BytesMut::new());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::JSON);
                assert_eq!(r.value, Some("{\"name\":\"Server\",\"version\":\"1.0\"}".to_string()));
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        }
        
        response_info = config::ResponseInfo::new(config::TEXT_FILE, None, Some("/file/path".to_string()));
        service_entry = config::ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None
        );
        let mut body = BytesMut::new();
        body.put(&b"{\"path\":\"/file/path\"}"[..]);
        let mut response = Server::preprocess(&service_entry, &body);
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::TEXT);
                assert_eq!(r.file, Some("/file/path".to_string()));
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        }
        
        response_info = config::ResponseInfo::new(config::BINARY_FILE, None, Some("/file/path".to_string()));
        service_entry = config::ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None
        );
        let mut body = BytesMut::new();
        body.put(&b"{\"path\":\"/file/path\"}"[..]);
        let mut response = Server::preprocess(&service_entry, &body);
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::BINARY);
                assert_eq!(r.file, Some("/file/path".to_string()));
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        }
        
        response_info = config::ResponseInfo::new(config::HANDLEBARS, None, Some("/file/path".to_string()));
        service_entry = config::ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None
        );
        let mut body = BytesMut::new();
        body.put(&b"{\"path\":\"/file/path\"}"[..]);
        let mut response = Server::preprocess(&service_entry, &body);
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::HTML);
                assert_eq!(r.file, Some("/file/path".to_string()));
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        }
   }

   #[test]
   fn test_preprocess_error() {
        let mut response_info = config::ResponseInfo::new(99, Some("Text goes here".to_string()), None);
        let mut service_entry = config::ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None
        );
        let mut response = Server::preprocess(&service_entry, &BytesMut::new());
        match response {
            Ok(_r) => assert_eq!(true, false),
            Err(e) => {
                assert_eq!(e.status(), server_status::INVALID_RESPONSE_CODE.status);
                assert_eq!(*(e.body()), server_status::INVALID_RESPONSE_CODE.name);
            }
        };    

        let mut response_info = config::ResponseInfo::new(config::TEXT_FILE, Some("Text goes here".to_string()), None);
        let mut service_entry = config::ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None
        );
        let mut response = Server::preprocess(&service_entry, &BytesMut::new());
        match response {
            Ok(_r) => assert_eq!(true, false),
            Err(e) => {
                assert_eq!(e.status(), server_status::INVALID_PATH.status);
                assert_eq!(*(e.body()), server_status::INVALID_PATH.name);
            }
        }
    }

    #[test]
    fn test_respond() {
        let request = http::request::Request::builder()
            .method("POST")
            .uri("https://www.rust-lang.org/index/0")
            .version(http::version::Version::HTTP_11)
            .body(BytesMut::new())
            .unwrap();
        let response = Server::respond(request);

        assert_eq!(response.status(), server_status::OK.status);
        assert_eq!(response.headers().contains_key("Content-Type"), true);
        assert_eq!(response.headers().get("Content-Type").unwrap().to_str().unwrap(), mime::JSON.mime_type.as_str());
        assert_eq!(response.body(), "{\"name\":\"U.S. Comics Server\",\"version\":\"0.0.1\"}");        
    }
}