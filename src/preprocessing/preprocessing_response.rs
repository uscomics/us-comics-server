use crate::config;
use crate::server_status;
use crate::util::mime;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PreprocessingResponse {
    pub status: server_status::ServerStatus,
    pub mime: mime::Mime,
    pub value: Option<String>,
    pub file: Option<String>,
    pub response_info: config::ResponseInfo
}

impl PreprocessingResponse {
    pub fn new(status: &server_status::ServerStatus, mime: &mime::Mime, value: Option<String>, file: Option<String>, response_info: &config::ResponseInfo) -> PreprocessingResponse { 
        PreprocessingResponse { status: status.clone(), mime: mime.clone(), value: value, file: file, response_info: response_info.clone() }
    }
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::config::*;
    use crate::preprocessing::preprocessing_response::*;
    use crate::server_status::*;
    use crate::util::mime::*;

    #[test]
    fn test_response() {
        let response_info = ResponseInfo::new(HANDLEBARS, Some("x=y;a=b;".to_string()), Some("file=a/b/c".to_string()));
        let response = PreprocessingResponse::new(&OK, &AAC_AUDIO, Some("value".to_string()), None, &response_info);
        assert_eq!(response.status.status, 200);
        assert_eq!(response.status.name, "OK");
        assert_eq!(response.status.context, "");
        assert_eq!(response.mime.mime_type, "audio/aac");
        assert_eq!(response.mime.ext, ".aac");
        assert_eq!(response.value, Some("value".to_string()));
        assert_eq!(response.file, None);
        assert_eq!(response.response_info.code, HANDLEBARS);
        assert_eq!(response.response_info.value, Some("x=y;a=b;".to_string()));
        assert_eq!(response.response_info.file, Some("file=a/b/c".to_string()));
    }
}