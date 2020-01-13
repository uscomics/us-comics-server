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
use crate::strings;

pub static DEFAULT_FIRECRACKER_PORT: u32 = 2;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server{}
    }

    pub fn init(config: &str) -> std::io::Result<config::ServerConfig> {
        println!("{}", config);
        let mut file = File::open(config)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let server_config: config::ServerConfig = serde_json::from_str(contents.as_str())?;
        let server_info_with_defaults = config::ServerInfo::add_defaults(&server_config.server);
        println!("{}", contents);
        let server_config_with_defaults = config::ServerConfig::new(&server_info_with_defaults, &server_config.services);
        Ok(server_config_with_defaults)
    }

    pub async fn start(config: &config::ServerConfig) -> Result<(), Box<dyn Error>> {
        // Parse the arguments, bind the TCP socket we'll be listening to, spin up
        // our worker threads, and start shipping sockets to those worker threads.
        let addr = Server::get_port(config);
        let mut server = TcpListener::bind(&addr).await?;
        let mut incoming = server.incoming();
        let listening_message = i18n::I18n::get_from_locale(strings::LISTENING_ON_PORT, i18n::DEFAULT_LOCALE, i18n::DEFAULT_PATH);
        println!("{} {}", listening_message, addr);

        while let Some(Ok(stream)) = incoming.next().await {
            tokio::spawn(async move {
                if let Err(e) = Server::read(stream).await {
                    println!("failed to read connection; error = {}", e);
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

    async fn respond(req: Request<()>) -> Result<Response<String>, Box<dyn Error>> {
        let mut response = Response::builder();
        let body = match req.uri().path() {
            "/plaintext" => {
                response = response.header("Content-Type", "text/plain");
                "Hello, World!".to_string()
            }
            "/json" => {
                response = response.header("Content-Type", "application/json");

                #[derive(Serialize)]
                struct Message {
                    message: &'static str,
                }
                serde_json::to_string(&Message {
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

    fn get_port(config: &config::ServerConfig) -> String {
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