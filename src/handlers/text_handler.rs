use bytes::BytesMut;

use crate::config;
use crate::mime;
use crate::handlers::handler_response::HandlerResponse;
use crate::server_status;

pub fn text_handler(service_entry: &config::ServiceEntry, _body: &mut BytesMut) -> Result<HandlerResponse, server_status::ServerStatus> {
    let value = match &service_entry.response_info.value {
        Some(value) => value,
        None => {
            let mut error = server_status::INVALID_VALUE.clone();
            error.context = format!("{:?}", service_entry.response_info);
            return Err(error);
        }
    };
    let response = HandlerResponse::new(&server_status::OK, &mime::TEXT, Some(value.clone()), None, &service_entry.response_info);
    Ok(response)
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::handlers::text_handler::*;
    use crate::config::*;

    #[test]
    fn test_text_handler() {
        let mut response_info = ResponseInfo::new(TEXT, Some("Text goes here".to_string()), None);
        let mut service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        let mut response = text_handler(&service_entry, &mut BytesMut::new());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::TEXT);
                assert_eq!(r.value, Some("Text goes here".to_string()));
                assert_eq!(r.response_info, response_info);        
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
        response = text_handler(&service_entry, &mut BytesMut::new());
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::TEXT);
                assert_eq!(r.value, Some("".to_string()));
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        }
    }

    #[test]
    fn test_text_handler_error() {
        let response_info = ResponseInfo::new(TEXT, None, None);
        let service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        let response = text_handler(&service_entry, &mut BytesMut::new());
        match response {
            Ok(_r) =>  assert_eq!(true, false),
            Err(e) => {
                assert_eq!(e.status, server_status::INVALID_VALUE.status);
                assert_eq!(e.name, server_status::INVALID_VALUE.name);
                assert_eq!(e.context, format!("{:?}", service_entry.response_info));        
            }
        }
    }
}