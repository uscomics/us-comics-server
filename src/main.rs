// https://blog.passcod.name/2018/mar/07/writing-servers-with-tokio
// https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
// https://stackoverflow.com/questions/53085270/how-do-i-implement-a-trait-with-a-generic-method

#![warn(rust_2018_idioms)]
use futures::SinkExt;
use http::{Request, Response, StatusCode};
use std::env;
use std::{error::Error, io};
use serde_json;
use tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed};
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

mod config;
mod http_codec;
mod i18n;
mod request;
mod router;
mod server_status;
mod strings;
mod url;
#[macro_use] mod log;

// A "tiny" example of HTTP request/response handling using transports.
//
// Code here is based on the `echo-threads` example and implements two paths,
// the `/plaintext` and `/json` routes to respond with some text and json,
// respectively. By default this will run I/O on all the cores your system has
// available, and it doesn't support HTTP request bodies.

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse the arguments, bind the TCP socket we'll be listening to, spin up
    // our worker threads, and start shipping sockets to those worker threads.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let mut server = TcpListener::bind(&addr).await?;
    let mut incoming = server.incoming();
    println!("Listening on: {}", addr);

    while let Some(Ok(stream)) = incoming.next().await {
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                println!("failed to process connection; error = {}", e);
            }
        });
    }

    Ok(())
}

async fn process(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut transport = Framed::new(stream, http_codec::Http);

    while let Some(request) = transport.next().await {
        match request {
            Ok(request) => {
                let response = respond(request).await?;
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