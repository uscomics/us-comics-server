use bytes::BytesMut;

use crate::preprocessing;
use crate::util::mime;
use crate::util::server_status;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessingResponse {
    pub status: server_status::ServerStatus,
    pub mime: mime::Mime,
    pub headers: http::header::HeaderMap,
    pub body: Option<BytesMut>,
    pub preprocessing_response: preprocessing::preprocessing_response::PreprocessingResponse
}

impl ProcessingResponse {
    pub fn new(status: &server_status::ServerStatus, mime: &mime::Mime, headers: &http::header::HeaderMap, body: &Option<BytesMut>, preprocessing_response: &preprocessing::preprocessing_response::PreprocessingResponse) -> ProcessingResponse { 
        ProcessingResponse { status: status.clone(), mime: mime.clone(), headers: headers.clone(), body: body.clone(), preprocessing_response: preprocessing_response.clone() }
    }

    pub fn copy_to_headers(&self, headers: &mut http::header::HeaderMap<http::header::HeaderValue>){
        for key in self.headers.keys() {
            match self.headers.get(key){
                Some(value) => headers.insert(key.clone(), value.clone()),
                None => None
            };
        }
    }
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use bytes::{BytesMut, BufMut};
    use crate::config::*;
    use crate::preprocessing::preprocessing_response::*;
    use crate::processing::processing_response::ProcessingResponse;
    use crate::util::server_status::*;
    use crate::util::mime::*;

    #[test]
    fn test_response() {
        let response_info = ResponseInfo::new(HANDLEBARS, Some("x=y;a=b;".to_string()), Some("file=a/b/c".to_string()));
        let preprocessing_response = PreprocessingResponse::new(&OK, &AAC_AUDIO, Some("value".to_string()), None, &response_info);
        let mut headers = http::header::HeaderMap::new();
        let mut body = BytesMut::new();
        let header_name = http::header::HeaderName::from_lowercase(b"header").unwrap();
        let value = "value";
        let header_value = http::header::HeaderValue::from_str(value).unwrap();
        headers.insert(header_name, header_value);
        body.put(&b"{\"path\":\"/cat/dog\"}"[..]);
        let response = ProcessingResponse::new(&OK, &AAC_AUDIO, &headers, &Some(body.clone()), &preprocessing_response);
        assert_eq!(response.status.status, 200);
        assert_eq!(response.status.name, "OK");
        assert_eq!(response.mime.mime_type, "audio/aac");
        assert_eq!(response.mime.ext, ".aac");
        assert_eq!(response.headers.len(), 1);
        assert_eq!(response.body, Some(body));
        assert_eq!(response.preprocessing_response, preprocessing_response);
    }
}
