// https://blog.passcod.name/2018/mar/07/writing-servers-with-tokio
// https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
// https://stackoverflow.com/questions/53085270/how-do-i-implement-a-trait-with-a-generic-method

#![warn(rust_2018_idioms)]
#![recursion_limit="256"]
use tokio;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

mod config;
mod http_codec;
mod preprocessing;
mod processing;
mod request;
mod router;
mod util;
mod server;

#[tokio::main]
async fn main() {
    server::Server::start().await;
}
