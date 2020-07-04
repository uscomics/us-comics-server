use crate::preprocessing::preprocessing_response::PreprocessingResponse;
use crate::processing::processing_response::ProcessingResponse;
use crate::util::mime;
use crate::util::server_status;


pub fn text_file_processor(preprocessing_response: &PreprocessingResponse) -> Result<ProcessingResponse, server_status::ServerStatus> {
    let mut headers = http::header::HeaderMap::new();
    let header_name = http::header::HeaderName::from_bytes("X-US-COMICS-FILE".as_bytes());
    let file = preprocessing_response.file.clone().unwrap();
    let header_value = http::header::HeaderValue::from_str(file.as_str()).unwrap();
    headers.insert(header_name.unwrap(), header_value);
    let response = ProcessingResponse::new(&server_status::OK, &mime::TEXT, &headers, &None, &preprocessing_response);
    Ok(response)
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use bytes::{BytesMut, BufMut};
    use crate::preprocessing::file_preprocessor::*;
    use crate::processing::text_file_processor::*;
    use crate::util::server_status;
    use crate::util::mime;
    use crate::config::*;

    #[test]
    fn test_text_file_processor() {
        let response_info = ResponseInfo::new(TEXT_FILE, None, Some("./path/to/:file".to_string()));
        let service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        let mut body = BytesMut::new();
        body.put(&b"{\"file\":\"my.file\"}"[..]);
        let preprocessor_response = file_preprocessor(&service_entry, &body).unwrap();
        let response = text_file_processor(&preprocessor_response);
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::TEXT);
                assert_eq!(r.headers.len(), 1);
                assert_eq!(r.headers.contains_key("X-US-COMICS-FILE"), true);
                assert_eq!(r.headers.get("X-US-COMICS-FILE").unwrap(), "./path/to/my.file");
                assert_eq!(r.body, None);
                assert_eq!(r.preprocessing_response, preprocessor_response);        
            },
            Err(_e) => assert_eq!(true, false)
        }
    }
}
