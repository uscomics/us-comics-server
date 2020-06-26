use bytes::{BytesMut, BufMut};

use crate::preprocessing::preprocessing_response::PreprocessingResponse;
use crate::processing::processing_response::ProcessingResponse;
use crate::util::mime;
use crate::util::server_status;

pub fn json_processor(preprocessing_response: &PreprocessingResponse) -> Result<ProcessingResponse, server_status::ServerStatus> {
    let value = match &preprocessing_response.value {
        Some(v) => v,
        None => {
            let mut error = server_status::INVALID_VALUE.clone();
            error.context = format!("{:?}", preprocessing_response.response_info);
            return Err(error);
        }
    };
    let mut headers = http::header::HeaderMap::new();
    let value = format!("{}", value.len());
    let header_value = http::header::HeaderValue::from_str(value.as_str()).unwrap();
    headers.insert(http::header::CONTENT_LENGTH, header_value);
    let mut body = BytesMut::with_capacity(value.len());
    body.put(value.as_bytes());
    let response = ProcessingResponse::new(&server_status::OK, &mime::JSON, &headers, &Some(body), &preprocessing_response);
    Ok(response)
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use bytes::BytesMut;
    use crate::preprocessing::json_preprocessor::*;
    use crate::processing::json_processor::*;
    use crate::util::server_status;
    use crate::util::mime;
    use crate::config::*;

    #[test]
    fn test_json_processor() {
        let mut response_info = ResponseInfo::new(TEXT, Some("name=Server;version=1.0;".to_string()), None);
        let mut service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        let mut preprocessor_response = json_preprocessor(&service_entry, &BytesMut::new()).unwrap();
        let mut response = json_processor(&preprocessor_response);
        let mut body = BytesMut::with_capacity(33);
        body.put("{\"name\":\"Server\",\"version\":\"1.0\"}".as_bytes());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::JSON);
                assert_eq!(r.headers.len(), 1);
                assert_eq!(r.headers.contains_key("Content-Length"), true);
                assert_eq!(r.headers.get("Content-Length").unwrap(), "33");
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
        preprocessor_response = json_preprocessor(&service_entry, &mut BytesMut::new()).unwrap();
        response = json_processor(&preprocessor_response);
        body = BytesMut::with_capacity(2);
        body.put("{}".as_bytes());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::JSON);
                assert_eq!(r.headers.len(), 1);
                assert_eq!(r.headers.contains_key("Content-Length"), true);
                assert_eq!(r.headers.get("Content-Length").unwrap(), "2");
                assert_eq!(r.body, Some(body));
                assert_eq!(r.preprocessing_response, preprocessor_response);        
            },
            Err(_e) => assert_eq!(true, false)
        }
    }
}