use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ServerStatus {
    pub status: u16,
    pub name: String
}
impl ServerStatus {
    pub fn new(status: u16, name: &str) -> ServerStatus { ServerStatus { status: status, name: name.to_string() }}
}
impl fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ \"status\": {}, \"name\": \"{}\" }}", self.status, self.name)
    }
}

#[allow(dead_code)]
lazy_static! {
    pub static ref OK: ServerStatus = ServerStatus::new(200, "OK");
    pub static ref CREATED: ServerStatus = ServerStatus::new(201, "Created");
    pub static ref ACCEPTED: ServerStatus = ServerStatus::new(202, "Accepted");
    pub static ref NON_AUTHORITATIVE_INFORMATION: ServerStatus = ServerStatus::new(203, "Non-Authoritative Information");
    pub static ref NO_CONTENT: ServerStatus = ServerStatus::new(204, "No Content");
    pub static ref RESET_CONTENT: ServerStatus = ServerStatus::new(205, "Reset Content");
    pub static ref PARTIAL_CONTENT: ServerStatus = ServerStatus::new(206, "Partial Content");
    pub static ref MULTI_STATUS: ServerStatus = ServerStatus::new(207, "Multi-Status");
    pub static ref ALREADY_REPORTED: ServerStatus = ServerStatus::new(208, "Already Reported");
    pub static ref IM_USED: ServerStatus = ServerStatus::new(226, "IM Used");

    pub static ref MULTIPLE_CHOICES: ServerStatus = ServerStatus::new(300, "Multiple Choices");
    pub static ref MOVED_PERMANENTLY: ServerStatus = ServerStatus::new(301, "Moved Permanently");
    pub static ref FOUND: ServerStatus = ServerStatus::new(302, "Found");
    pub static ref SEE_OTHER: ServerStatus = ServerStatus::new(303, "See Other");
    pub static ref NOT_MODIFIED: ServerStatus = ServerStatus::new(304, "Not Modified");
    pub static ref USE_PROXY: ServerStatus = ServerStatus::new(305, "Use Proxy");
    pub static ref SWITCH_PROXY: ServerStatus = ServerStatus::new(306, "Switch Proxy");
    pub static ref TEMPORARY_REDIRECT: ServerStatus = ServerStatus::new(307, "Temporary Redirect");
    pub static ref PERMANENT_REDIRECT: ServerStatus = ServerStatus::new(308, "Permanent Redirect");

    pub static ref BAD_REQUEST: ServerStatus = ServerStatus::new(400, "Bad Request");
    pub static ref UNAUTHORIZED: ServerStatus = ServerStatus::new(401, "Unauthorized");
    pub static ref PAYMENT_REQUIRED: ServerStatus = ServerStatus::new(402, "Payment Required");
    pub static ref FORBIDDEN: ServerStatus = ServerStatus::new(403, "Forbidden");
    pub static ref NOT_FOUND: ServerStatus = ServerStatus::new(404, "Not Found");
    pub static ref METHOD_NOT_ALLOWED: ServerStatus = ServerStatus::new(405, "Method Not Allowed");
    pub static ref NOT_ACCEPTABLE: ServerStatus = ServerStatus::new(406, "Not Acceptable");
    pub static ref PROXY_AUTHENTICATION_REQUIRED: ServerStatus = ServerStatus::new(407, "Proxy Authentication Required");
    pub static ref REQUEST_TIMEOUT: ServerStatus = ServerStatus::new(408, "Request Timeout");
    pub static ref CONFLICT: ServerStatus = ServerStatus::new(409, "Conflict");
    pub static ref GONE: ServerStatus = ServerStatus::new(410, "Gone");
    pub static ref LENGTH_REQUIRED: ServerStatus = ServerStatus::new(411, "Length Required");
    pub static ref PRECONDITION_FAILED: ServerStatus = ServerStatus::new(412, "Precondition Failed");
    pub static ref PAYLOAD_TOO_LARGE: ServerStatus = ServerStatus::new(413, "Payload Too Large");
    pub static ref URI_TOO_LONG: ServerStatus = ServerStatus::new(414, "URI Too Long");
    pub static ref UNSUPPORTED_MEDIA_TYPE: ServerStatus = ServerStatus::new(415, "Unsupported Media Type");
    pub static ref RANGE_NOT_SATISFIABLE: ServerStatus = ServerStatus::new(416, "Range Not Satisfiable");
    pub static ref EXPECTATION_FAILED: ServerStatus = ServerStatus::new(417, "Expectation Failed");
    pub static ref MISDIRECTED_REQUEST: ServerStatus = ServerStatus::new(421, "Misdirected Request");
    pub static ref UNPROCESSABLE_ENTITY: ServerStatus = ServerStatus::new(422, "Unprocessable Entity");
    pub static ref LOCKED: ServerStatus = ServerStatus::new(423, "Locked");
    pub static ref FAILED_DEPENDENCY: ServerStatus = ServerStatus::new(424, "Failed Dependency");
    pub static ref TOO_EARLY: ServerStatus = ServerStatus::new(425, "Too Early");
    pub static ref UPGRADE_REQUIRED: ServerStatus = ServerStatus::new(426, "Upgrade Required");
    pub static ref PRECONDITION_REQUIRED: ServerStatus = ServerStatus::new(428, "Precondition Required");
    pub static ref TOO_MANY_REQUESTS: ServerStatus = ServerStatus::new(429, "Too Many Requests");
    pub static ref REQUEST_HEADER_FIELDS_TOO_LARGE: ServerStatus = ServerStatus::new(431, "Request Header Fields Too Large");
    pub static ref UNAVAILABLE_FOR_LEGAL_REASONS: ServerStatus = ServerStatus::new(451, "Unavailable For Legal Reasons");
    pub static ref COULD_NOT_READ_HEADER: ServerStatus = ServerStatus::new(498, "Could not read header");
    pub static ref COULD_NOT_PARSE_HTTP_REQUEST: ServerStatus = ServerStatus::new(499, "Could not parse HTTP request");

    pub static ref INTERNAL_SERVER_ERROR: ServerStatus = ServerStatus::new(500, "Internal Server Error");
    pub static ref NOT_IMPLEMENTED: ServerStatus = ServerStatus::new(501, "Not Implemented");
    pub static ref BAD_GATEWAY: ServerStatus = ServerStatus::new(502, "Bad Gateway");
    pub static ref SERVICE_UNAVAILABLE: ServerStatus = ServerStatus::new(503, "Service Unavailable");
    pub static ref GATEWAY_TIMEOUT: ServerStatus = ServerStatus::new(504, "Gateway Timeout");
    pub static ref HTTP_VERSION_NOT_SUPPORTED: ServerStatus = ServerStatus::new(505, "HTTP Version Not Supported");
    pub static ref VARIANT_ALSO_NEGOTIATES: ServerStatus = ServerStatus::new(506, "Variant Also Negotiates");
    pub static ref INSUFFICIENT_STORAGE: ServerStatus = ServerStatus::new(507, "Insufficient Storage");
    pub static ref LOOP_DETECTED: ServerStatus = ServerStatus::new(508, "Loop Detected");
    pub static ref NOT_EXTENDED: ServerStatus = ServerStatus::new(510, "Not Extended");
    pub static ref NETWORK_AUTHENTICATION_REQUIRED: ServerStatus = ServerStatus::new(511, "Network Authentication Required");
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::server_status::*;

    #[test]
    fn test_status() {
        let status = ServerStatus::new(200, "OK");
        assert_eq!(status.status, 200);
        assert_eq!(status.name, "OK");
    }
}