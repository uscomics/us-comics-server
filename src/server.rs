use bytes::BytesMut;
use chrono::prelude::*;
use console::style;
use futures::SinkExt;
use http::{Request, Response, StatusCode};
use serde_json;
use std::{error::Error, io};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed};

use crate::config;
use crate::http_codec;
use crate::i18n;
use crate::log;
use crate::{structured_debug, structured_info, structured_error, structured_log, style_text};
use crate::strings;

pub static DEFAULT_FIRECRACKER_PORT: u32 = 2;

pub struct Server {
    config: config::ServerConfig,
    i18n: i18n::I18n,
    log: log::Log
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

    pub async fn start(server: &Server) -> Result<(), Box<dyn Error>> {
        // Parse the arguments, bind the TCP socket we'll be listening to, spin up
        // our worker threads, and start shipping sockets to those worker threads.
        let addr = Server::get_firecracker_address(&server.config);
        let mut tokio_server = TcpListener::bind(&addr).await?;
        let mut incoming = tokio_server.incoming();
        let listening_message = server.i18n.get(strings::FIRECRACKER_LISTENING_ON_PORT);
        structured_info!( server.log, "{} {}", listening_message, addr);

        while let Some(Ok(stream)) = incoming.next().await {
            let (locale, path, log_level) = (server.i18n.locale.clone(), server.i18n.path.clone(), server.log.level.clone());
            tokio::spawn(async move {
                if let Err(e) = Server::read(stream).await {
                    let i18n = i18n::I18n::new(locale.as_str(), path.as_str());
                    let error_message = i18n.get(strings::REQUEST_NOT_READ);
                    let log = log::Log::new(log_level, &i18n);
                    structured_error!( log, "{} {:?}", error_message, e);
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
                    let response = Server::respond(request).await?;
                    transport.send(response).await?;
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    async fn respond(req: Request<BytesMut>) -> Result<Response<String>, Box<dyn Error>> {
        let mut response = Response::builder();
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
        let response = response
            .body(body)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(response)
    }

    fn get_firecracker_address(config: &config::ServerConfig) -> String {
        let port = match config.server.port {
            Some(port) => port,
            None => {
                for (key, value) in env::vars() {
                    if key == ("FIRECRACKER_PORT") { value.parse::<u32>().unwrap_or(DEFAULT_FIRECRACKER_PORT); }
                }
                DEFAULT_FIRECRACKER_PORT
            }
        };
        format!("{}{}", "127.0.0.1:", port)
    }
}