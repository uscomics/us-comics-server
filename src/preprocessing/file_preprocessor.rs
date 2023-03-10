use bytes::BytesMut;
use serde_json::Value;
use std::str;

use crate::config;
use crate::preprocessing::preprocessing_response::PreprocessingResponse;
use crate::util::server_status;
use crate::util::mime;

// Body contains the requested file and service_entry.response_info.file maps that request to an actual file.
pub fn file_preprocessor(service_entry: &config::ServiceEntry, body: &BytesMut) -> Result<PreprocessingResponse, server_status::ServerStatus> {
    // Validate the response file.
    let response_file = match &service_entry.response_info.file {
        Some(rf) => {
            if 0 == rf.len() {
                let mut error = server_status::INVALID_FILE.clone();
                error.context = format!("{:?}", service_entry.response_info);
                return Err(error);
            }
            rf
        },
        None => {
            let mut error = server_status::INVALID_FILE.clone();
            error.context = format!("{:?}", service_entry.response_info);
            return Err(error);
        }
    };

    // Validate the requested file.
    let requested_file_json = match str::from_utf8(body) {
        Ok(v) => {
            if 0 == v.len() {
                let mut error = server_status::INVALID_FILE.clone();
                error.context = format!("{:?}", service_entry.response_info);
                return Err(error);
            }
            v
        },
        Err(_e) => {
            let mut err = server_status::INVALID_FILE.clone();
            err.context = format!("{:?}", service_entry.response_info);
            return Err(err);
        }
    };
    let parsed: Value = match serde_json::from_str(requested_file_json) {
        Ok(v) => {
            v
        },
        Err(_e) => {
            let mut err = server_status::INVALID_FILE.clone();
            err.context = format!("{:?}", service_entry.response_info);
            return Err(err);
        }
    };
    let file = match &parsed["file"]{
        Value::String(p) => p,
        _ => {
            let mut error = server_status::INVALID_FILE.clone();
            error.context = format!("{:?}", service_entry.response_info);
            return Err(error);
        }
    };
    let requested_path = response_file.replace(":file", file);

    // Get MIME type
    let mime_type: mime::Mime = match service_entry.response_info.code {
        config::BINARY_FILE => mime::BINARY.clone(),
        config::TEXT_FILE => mime::TEXT.clone(),
        config::HANDLEBARS => mime::HTML.clone(),
        _ => {
            let mut error = server_status::INVALID_RESPONSE_CODE.clone();
            error.context = format!("{:?}", service_entry.response_info);
            return Err(error);    
        }
    };

    // Build JSON parameters
    let value = match service_entry.response_info.value.clone() {
        Some(v)=> {
            match config::to_json(&v.as_str()) {
                Ok(j) => Some(j),
                _ => {
                    let mut error = server_status::INVALID_KEY_VALUE_PAIRS.clone();
                    error.context = format!("{:?}", service_entry.response_info);
                    return Err(error);    
                }
            }
        },
        None => None
    };

    // Return response.
    let response = PreprocessingResponse::new(&server_status::OK, &mime_type, value, Some(requested_path.clone()), &service_entry.response_info);
    Ok(response)
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::preprocessing::file_preprocessor::*;
    use crate::config::*;
    use bytes::{BytesMut, BufMut};
    use crate::util::server_status;
    use crate::util::mime;

    #[test]
    fn test_file_preprocessor() {
        let mut response_info = ResponseInfo::new(TEXT_FILE, None, Some("./path/to/:file".to_string()));
        let mut service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        let mut body = BytesMut::new();
        body.put(&b"{\"file\":\"my.file\"}"[..]);
        let mut response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::TEXT);
                assert_eq!(r.file, Some("./path/to/my.file".to_string()));
                assert_eq!(r.value, None);
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        };
        response_info = ResponseInfo::new(BINARY_FILE, None, Some("./path/to/:file".to_string()));
        service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        body = BytesMut::new();
        body.put(&b"{\"file\":\"my.file\"}"[..]);
        response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::BINARY);
                assert_eq!(r.file, Some("./path/to/my.file".to_string()));
                assert_eq!(r.value, None);
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        };
        response_info = ResponseInfo::new(HANDLEBARS, None, Some("./path/to/my.file".to_string()));
        service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        body = BytesMut::new();
        body.put(&b"{\"file\":\"my.file\"}"[..]);
        response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::HTML);
                assert_eq!(r.file, Some("./path/to/my.file".to_string()));
                assert_eq!(r.value, None);
                assert_eq!(r.response_info, response_info);        
            },
            Err(_e) => assert_eq!(true, false)
        };
    }
    
    #[test]
    fn test_file_preprocessor_with_params() {
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
        let response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::TEXT);
                assert_eq!(r.file, Some("./path/to/my.file".to_string()));
                assert_eq!(r.value, None);
                assert_eq!(r.response_info, response_info);     
            },
            Err(_e) => assert_eq!(true, false)
        };
    }
    
    #[test]
    fn test_file_preprocessor_with_values() {
        let response_info = ResponseInfo::new(TEXT_FILE, Some("title=Handlebars template data;goes=here;".to_string()), Some("./path/to/:file".to_string()));
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
        let response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(r) => {
                assert_eq!(r.status, *server_status::OK);
                assert_eq!(r.mime, *mime::TEXT);
                assert_eq!(r.file, Some("./path/to/my.file".to_string()));
                assert_eq!(r.value, Some("{\"title\":\"Handlebars template data\",\"goes\":\"here\"}".to_string()));
                assert_eq!(r.response_info, response_info);     
            },
            Err(_e) => assert_eq!(true, false)
        };
    }
    
    #[test]
    fn test_file_preprocessor_bad_response_file() {
        let mut response_info = ResponseInfo::new(TEXT_FILE, None, Some("".to_string()));
        let mut service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        let mut body = BytesMut::new();
        body.put(&b"{\"file\":\"my.file\"}"[..]);
        let mut response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(_r) => assert_eq!(true, false),
            Err(e) => {
                assert_eq!(e.status, server_status::INVALID_FILE.status);
                assert_eq!(e.name, server_status::INVALID_FILE.name);
                assert_eq!(e.context, format!("{:?}", response_info));
            }
        };
        response_info = ResponseInfo::new(TEXT_FILE, None, None);
        service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        body = BytesMut::new();
        body.put(&b"{\"file\":\"my.file\"}"[..]);
        response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(_r) => assert_eq!(true, false),
            Err(e) => {
                assert_eq!(e.status, server_status::INVALID_FILE.status);
                assert_eq!(e.name, server_status::INVALID_FILE.name);
                assert_eq!(e.context, format!("{:?}", response_info));
            }
        };    
    }
    
    #[test]
    fn test_file_preprocessor_bad_body() {
        let mut response_info = ResponseInfo::new(TEXT_FILE, None, Some("./path/to/:file".to_string()));
        let mut service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        let mut body = BytesMut::new();
        let mut response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(_r) => assert_eq!(true, false),
            Err(e) => {
                assert_eq!(e.status, server_status::INVALID_FILE.status);
                assert_eq!(e.name, server_status::INVALID_FILE.name);
                assert_eq!(e.context, format!("{:?}", response_info));
            }
        }; 
        response_info = ResponseInfo::new(TEXT_FILE, None, Some("./path/to/:file".to_string()));
        service_entry = ServiceEntry::new(
            0, 
            "name", 
            "description", 
            &response_info, 
            &None, 
            &None, 
            &None);
        body = BytesMut::new();
        body.put(&b"{\"not_file\":\"/file/path\"}"[..]);
        response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(_r) => assert_eq!(true, false),
            Err(e) => {
                assert_eq!(e.status, server_status::INVALID_FILE.status);
                assert_eq!(e.name, server_status::INVALID_FILE.name);
                assert_eq!(e.context, format!("{:?}", response_info));
            }
        };    
    }
    
    #[test]
    fn test_file_preprocessor_with_bad_response_code() {
        let response_info = ResponseInfo::new(99, None, Some("./path/to/:file".to_string()));
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
        let response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(_r) => assert_eq!(true, false),
            Err(e) => {
                assert_eq!(e.status, server_status::INVALID_RESPONSE_CODE.status);
                assert_eq!(e.name, server_status::INVALID_RESPONSE_CODE.name);
                assert_eq!(e.context, format!("{:?}", response_info));
            }
        };    
    }
    
    #[test]
    fn test_file_preprocessor_with_bad_values() {
        let response_info = ResponseInfo::new(TEXT_FILE, Some("Whatever".to_string()), Some("./path/to/:file".to_string()));
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
        let response = file_preprocessor(&service_entry, &body);
        match response {
            Ok(_r) => assert_eq!(true, false),
            Err(e) => {
                assert_eq!(e.status, server_status::INVALID_KEY_VALUE_PAIRS.status);
                assert_eq!(e.name, server_status::INVALID_KEY_VALUE_PAIRS.name);
                assert_eq!(e.context, format!("{:?}", response_info));
            }
        };
    }
}
