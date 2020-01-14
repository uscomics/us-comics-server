use http::{Request, Response, StatusCode};

use crate::config;

pub fn ping_handler(service_entry: &ServiceEntry, body: Option<()>) -> Result<Response<String>, Box<dyn Error>> {
    let mut response = Response::builder();
    response = response.header("Content-Type", "application/json");
    let body = config::parse_values(ServiceEntry.response.value);
    let response = response
        .body(body)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    Ok(response)
}