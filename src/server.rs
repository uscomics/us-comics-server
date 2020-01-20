use bytes::BytesMut;
use chrono::prelude::*;
use console::style;
use futures::SinkExt;
use http::{Request, Response, StatusCode};
use serde_json;
use std::cell::Cell;
use std::collections::HashMap;
use std::{error::Error, io};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed};

use crate::config;
use crate::http_codec;
use crate::util::*;
use crate::{structured_debug, structured_info, structured_error, structured_log, style_text};

pub static DEFAULT_PORT: u32 = 2;

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
        let addr = Server::get_address(&CONFIG.config);
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
                    let response = Server::respond(request).await;
                    transport.send(response).await?;
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    async fn respond(req: Request<BytesMut>) -> Response<String> {
        let mut builder = Response::builder();
        let url = req.uri().path();
        let mut params = HashMap::new();
        url::get_params(url, "/index/:id", &mut params);
        let id = match(params["id"].parse::<usize>()) {
            Ok(i) => i,
            Err(e) => {
                let error_message = server_status::COULD_NOT_PARSE_HTTP_REQUEST.clone();
                structured_error!( CONFIG.log, "{:?} {:?}", error_message, e);
                return builder.status(server_status::COULD_NOT_PARSE_HTTP_REQUEST.status)
                    .body(server_status::COULD_NOT_PARSE_HTTP_REQUEST.name.clone()).unwrap();
            }
        };
        if CONFIG.config.services.len() <= id {
            let error_message = server_status::INVALID_SERVICE.clone();
            structured_error!( CONFIG.log, "{:?}: {}", error_message, id);
            return builder.status(server_status::INVALID_SERVICE.status)
                .body(server_status::INVALID_SERVICE.name.clone()).unwrap();
        }

        let mut body = "".to_string();
        if 0 == id {
            builder = builder.header("Content-Type", "application/json");
            body = "ping".to_string();
        }
        /*
        let body = match req.uri().path() {
            "/plaintext" => {
                response = response.header("Content-Type", "text/plain");
                "Hello, World!".to_string()
            }
            "/json" => {
                let req_body = req.body();
                println!("{:?}", req_body);
                response = response.header("Content-Type", "application/json");

                #[derive(Serialize)]
                struct Message {
                    message: &'static str,
                    id: Option<i32>,
                    title: String,
                    body: String,
                    #[serde(rename = "userId")] 
                    user_id: i32    
                }   
                serde_json::to_string(&Message {    
                    id: Some(42),
                    title: "Title".to_string(),
                    body: "Body".to_string(),
                    user_id: 42,
                    message: "Hello, World!",
                })?
            }
            _ => {
                response = response.status(StatusCode::NOT_FOUND);
                String::new()
            }
        };
        */
        let response = builder.body(body).unwrap();

        response
    }

    fn get_address(config: &config::ServerConfig) -> String {
        let port = match config.server.port {
            Some(port) => port,
            None => {
                for (key, value) in env::vars() {
                    if key == ("US_COMICS_PORT") { value.parse::<u32>().unwrap_or(DEFAULT_PORT); }
                }
                DEFAULT_PORT
            }
        };
        format!("{}{}", "127.0.0.1:", port)
    }
}