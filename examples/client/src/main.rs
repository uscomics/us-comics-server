use http_req::request;

fn main() {
    let mut writer = Vec::new(); //container for body of a response
    const BODY: &[u8; 24] = b"{\"file\": \"Cargo.toml\"}";
    let res = request::post("http://localhost:8080/index/4", BODY, &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Headers {}", res.headers());
    println!("{}", String::from_utf8_lossy(&writer));
}   
