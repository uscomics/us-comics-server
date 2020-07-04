use bytes::{BytesMut, BufMut};

use crate::preprocessing::preprocessing_response::PreprocessingResponse;
use crate::processing::processing_response::ProcessingResponse;
use crate::util::mime;
use crate::util::server_status;


pub fn text_processor(preprocessing_response: &PreprocessingResponse) -> Result<ProcessingResponse, server_status::ServerStatus> {
    let preprocessing_value = match &preprocessing_response.value {
        Some(v) => v,
        None => {
            let mut error = server_status::INVALID_VALUE.clone();
            error.context = format!("{:?}", preprocessing_response.response_info);
            return Err(error);
        }
    };
    let mut headers = http::header::HeaderMap::new();
    let header_name = http::header::HeaderName::from_lowercase(b"content-length").unwrap();
    let value = format!("{}", preprocessing_value.len());
    let header_value = http::header::HeaderValue::from_str(value.as_str()).unwrap();
    headers.insert(header_name, header_value);
    let mut body = BytesMut::with_capacity(preprocessing_value.len());
    body.put(preprocessing_value.as_bytes());
    let response = ProcessingResponse::new(&server_status::OK, &mime::TEXT, &headers, &Some(body), &preprocessing_response);
    Ok(response)
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use bytes::BytesMut;
    use crate::preprocessing::text_preprocessor::*;
    use crate::processing::text_processor::*;
    use crate::util::server_status;
    use crate::util::mime;
    use crate::config::*;

    #[test]
    fn test_text_processor() {
        let mut response_info = ResponseInfo::new(TEXT, Some("Text goes here".to_string()), None);
        let mut service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        let mut preprocessor_response = text_preprocessor(&service_entry, &BytesMut::new()).unwrap();
        let mut response = text_processor(&preprocessor_response);
        let mut body = BytesMut::with_capacity(14);
        body.put("Text goes here".as_bytes());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::TEXT);
                assert_eq!(r.headers.len(), 1);
                assert_eq!(r.headers.contains_key("Content-Length"), true);
                assert_eq!(r.headers.get("Content-Length").unwrap(), "14");
                assert_eq!(r.body, Some(body));
                assert_eq!(r.preprocessing_response, preprocessor_response);        
            },
            Err(_e) => assert_eq!(true, false)
        }

        response_info = ResponseInfo::new(TEXT, Some("".to_string()), None);
        service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        preprocessor_response = text_preprocessor(&service_entry, &mut BytesMut::new()).unwrap();
        response = text_processor(&preprocessor_response);
        body = BytesMut::with_capacity(1);
        body.put("".as_bytes());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::TEXT);
                assert_eq!(r.headers.len(), 1);
                assert_eq!(r.headers.contains_key("Content-Length"), true);
                assert_eq!(r.headers.get("Content-Length").unwrap(), "0");
                assert_eq!(r.body, Some(body));
                assert_eq!(r.preprocessing_response, preprocessor_response);        
            },
            Err(_e) => assert_eq!(true, false)
        }
    }
}