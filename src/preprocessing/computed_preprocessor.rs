use bytes::BytesMut;

use crate::config;
use crate::util::mime;
use crate::preprocessing::preprocessing_response::PreprocessingResponse;
use crate::util::server_status;

// Computed is for custom code, so we leave it to the processor to deal with the data. We only preprocess it so the 
// pipeline is consistant across all response codes.
pub fn computed_preprocessor(service_entry: &config::ServiceEntry, _body: &BytesMut) -> Result<PreprocessingResponse, server_status::ServerStatus> {
    let response = PreprocessingResponse::new(
        &server_status::OK, 
        &mime::BINARY, 
        service_entry.response_info.value.clone(), 
        service_entry.response_info.file.clone(), 
        &service_entry.response_info
    );
    Ok(response)
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::preprocessing::computed_preprocessor::*;
    use crate::config::*;

    #[test]
    fn test_computed_preprocessor() {
        let mut response_info = ResponseInfo::new(COMPUTED, Some("Text goes here".to_string()), None);
        let mut service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        let mut response = computed_preprocessor(&service_entry, &mut BytesMut::new());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::BINARY);
                assert_eq!(r.value, Some("Text goes here".to_string()));
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        }

        response_info = ResponseInfo::new(COMPUTED, Some("".to_string()), None);
        service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        response = computed_preprocessor(&service_entry, &mut BytesMut::new());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::BINARY);
                assert_eq!(r.value, Some("".to_string()));
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        }

        response_info = ResponseInfo::new(COMPUTED, None, None);
        service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        response = computed_preprocessor(&service_entry, &mut BytesMut::new());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::BINARY);
                assert_eq!(r.value, None);
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        }
    }
}