// https://blog.passcod.name/2018/mar/07/writing-servers-with-tokio
// https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
// https://stackoverflow.com/questions/53085270/how-do-i-implement-a-trait-with-a-generic-method

#![warn(rust_2018_idioms)]
use tokio;
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
mod server;
#[macro_use] mod log;

// A "tiny" example of HTTP request/response handling using transports.
//
// Code here is based on the `echo-threads` example and implements two paths,
// the `/plaintext` and `/json` routes to respond with some text and json,
// respectively. By default this will run I/O on all the cores your system has
// available, and it doesn't support HTTP request bodies.
#[tokio::main]
async fn main() {
    let config_path = "./config/config.json";
    match server::Server::init(config_path) {
        Ok(config) => {
            let firecracker = config;
            server::Server::start(&firecracker).await;
        },
        Err(e) => {
            println!("Bad config: {}", e); 
        }
    };
}
