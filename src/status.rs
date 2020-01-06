use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Status {
    pub status: u16,
    pub name: String
}
impl Status {
    pub fn new(status: u16, name: &str) -> Status { Status { status: status, name: name.to_string() }}
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ \"status\": {}, \"name\": \"{}\" }}", self.status, self.name)
    }
}

#[allow(dead_code)]
lazy_static! {
    pub static ref OK: Status = Status::new(200, "OK");
    pub static ref CREATED: Status = Status::new(201, "Created");
    pub static ref ACCEPTED: Status = Status::new(202, "Accepted");
    pub static ref NON_AUTHORITATIVE_INFORMATION: Status = Status::new(203, "Non-Authoritative Information");
    pub static ref NO_CONTENT: Status = Status::new(204, "No Content");
    pub static ref RESET_CONTENT: Status = Status::new(205, "Reset Content");
    pub static ref PARTIAL_CONTENT: Status = Status::new(206, "Partial Content");
    pub static ref MULTI_STATUS: Status = Status::new(207, "Multi-Status");
    pub static ref ALREADY_REPORTED: Status = Status::new(208, "Already Reported");
    pub static ref IM_USED: Status = Status::new(226, "IM Used");

    pub static ref MULTIPLE_CHOICES: Status = Status::new(300, "Multiple Choices");
    pub static ref MOVED_PERMANENTLY: Status = Status::new(301, "Moved Permanently");
    pub static ref FOUND: Status = Status::new(302, "Found");
    pub static ref SEE_OTHER: Status = Status::new(303, "See Other");
    pub static ref NOT_MODIFIED: Status = Status::new(304, "Not Modified");
    pub static ref USE_PROXY: Status = Status::new(305, "Use Proxy");
    pub static ref SWITCH_PROXY: Status = Status::new(306, "Switch Proxy");
    pub static ref TEMPORARY_REDIRECT: Status = Status::new(307, "Temporary Redirect");
    pub static ref PERMANENT_REDIRECT: Status = Status::new(308, "Permanent Redirect");

    pub static ref BAD_REQUEST: Status = Status::new(400, "Bad Request");
    pub static ref UNAUTHORIZED: Status = Status::new(401, "Unauthorized");
    pub static ref PAYMENT_REQUIRED: Status = Status::new(402, "Payment Required");
    pub static ref FORBIDDEN: Status = Status::new(403, "Forbidden");
    pub static ref NOT_FOUND: Status = Status::new(404, "Not Found");
    pub static ref METHOD_NOT_ALLOWED: Status = Status::new(405, "Method Not Allowed");
    pub static ref NOT_ACCEPTABLE: Status = Status::new(406, "Not Acceptable");
    pub static ref PROXY_AUTHENTICATION_REQUIRED: Status = Status::new(407, "Proxy Authentication Required");
    pub static ref REQUEST_TIMEOUT: Status = Status::new(408, "Request Timeout");
    pub static ref CONFLICT: Status = Status::new(409, "Conflict");
    pub static ref GONE: Status = Status::new(410, "Gone");
    pub static ref LENGTH_REQUIRED: Status = Status::new(411, "Length Required");
    pub static ref PRECONDITION_FAILED: Status = Status::new(412, "Precondition Failed");
    pub static ref PAYLOAD_TOO_LARGE: Status = Status::new(413, "Payload Too Large");
    pub static ref URI_TOO_LONG: Status = Status::new(414, "URI Too Long");
    pub static ref UNSUPPORTED_MEDIA_TYPE: Status = Status::new(415, "Unsupported Media Type");
    pub static ref RANGE_NOT_SATISFIABLE: Status = Status::new(416, "Range Not Satisfiable");
    pub static ref EXPECTATION_FAILED: Status = Status::new(417, "Expectation Failed");
    pub static ref MISDIRECTED_REQUEST: Status = Status::new(421, "Misdirected Request");
    pub static ref UNPROCESSABLE_ENTITY: Status = Status::new(422, "Unprocessable Entity");
    pub static ref LOCKED: Status = Status::new(423, "Locked");
    pub static ref FAILED_DEPENDENCY: Status = Status::new(424, "Failed Dependency");
    pub static ref TOO_EARLY: Status = Status::new(425, "Too Early");
    pub static ref UPGRADE_REQUIRED: Status = Status::new(426, "Upgrade Required");
    pub static ref PRECONDITION_REQUIRED: Status = Status::new(428, "Precondition Required");
    pub static ref TOO_MANY_REQUESTS: Status = Status::new(429, "Too Many Requests");
    pub static ref REQUEST_HEADER_FIELDS_TOO_LARGE: Status = Status::new(431, "Request Header Fields Too Large");
    pub static ref UNAVAILABLE_FOR_LEGAL_REASONS: Status = Status::new(451, "Unavailable For Legal Reasons");

    pub static ref INTERNAL_SERVER_ERROR: Status = Status::new(500, "Internal Server Error");
    pub static ref NOT_IMPLEMENTED: Status = Status::new(501, "Not Implemented");
    pub static ref BAD_GATEWAY: Status = Status::new(502, "Bad Gateway");
    pub static ref SERVICE_UNAVAILABLE: Status = Status::new(503, "Service Unavailable");
    pub static ref GATEWAY_TIMEOUT: Status = Status::new(504, "Gateway Timeout");
    pub static ref HTTP_VERSION_NOT_SUPPORTED: Status = Status::new(505, "HTTP Version Not Supported");
    pub static ref VARIANT_ALSO_NEGOTIATES: Status = Status::new(506, "Variant Also Negotiates");
    pub static ref INSUFFICIENT_STORAGE: Status = Status::new(507, "Insufficient Storage");
    pub static ref LOOP_DETECTED: Status = Status::new(508, "Loop Detected");
    pub static ref NOT_EXTENDED: Status = Status::new(510, "Not Extended");
    pub static ref NETWORK_AUTHENTICATION_REQUIRED: Status = Status::new(511, "Network Authentication Required");
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::status::*;

    #[test]
    fn test_status() {
        let status = Status::new(200, "OK");
        assert_eq!(status.status, 200);
        assert_eq!(status.name, "OK");
    }
}