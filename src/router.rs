pub mod router {
    // Errors
    pub struct RouterError<'a> {
        pub status: u16,
        pub message: &'a str
    }
    impl<'a> RouterError<'a> {
        pub fn new(status: u16, message: &'a str) -> RouterError<'a> { RouterError { status: status, message: message }}
    }
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use super::router::*;

    #[test]
    fn test_router_error() {
        let router_error = RouterError::new(200, "OK");
        assert_eq!(router_error.status, 200);
        assert_eq!(router_error.message, "OK");
    }
}