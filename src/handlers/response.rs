use crate::config;
use crate::mime;
use crate::server_status;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Response {
    pub status: server_status::ServerStatus,
    pub mime: mime::Mime,
    pub value: String,
    pub response_info: config::ResponseInfo
}

impl Response {
    pub fn new(status: &server_status::ServerStatus, mime: &mime::Mime, value: &str, response_info: &config::ResponseInfo) -> Response { 
        Response { status: status.clone(), mime: mime.clone(), value: value.to_string(), response_info: response_info.clone() }
    }
}


///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::config::*;
    use crate::handlers::response::*;
    use crate::server_status::*;
    use crate::mime::*;

    #[test]
    fn test_response() {
        let response_info = ResponseInfo::new(HANDLEBARS, Some("x=y;a=b;".to_string()), Some("file=a/b/c".to_string()));
        let response = Response::new(&OK, &AAC_AUDIO, "value", &response_info);
        assert_eq!(response.status.status, 200);
        assert_eq!(response.status.name, "OK");
        assert_eq!(response.status.context, "");
        assert_eq!(response.mime.mime_type, "audio/aac");
        assert_eq!(response.mime.ext, ".aac");
        assert_eq!(response.value, "value");
        assert_eq!(response.response_info.code, HANDLEBARS);
        assert_eq!(response.response_info.value, Some("x=y;a=b;".to_string()));
        assert_eq!(response.response_info.file, Some("file=a/b/c".to_string()));
    }
}