pub mod config {
    // Used to store an entry from the config file.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConfigRequest<'a> {
        pub name: &'a str,          // Entry name.
        pub description: &'a str,   // Entry description.
        pub route: &'a str,         // Entry route.
        pub parser: &'a str,        // Parser name.
        pub handler: &'a str,       // Handler name.
    }
    impl<'a> ConfigRequest<'a> {
        pub fn new(name: &'a str, description: &'a str, route: &'a str, parser: &'a str, handler: &'a str) -> ConfigRequest<'a> {
            let config = ConfigRequest {
                name: name,
                description: description,
                route: route,
                parser: parser,
                handler: handler
            };
            config
        }
    }
}
    
///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use super::config::*;

    #[test]
    fn test_new_config_request() {
        let config_request = ConfigRequest::new( "name", "description", "route", "parser", "handler");
        assert_eq!(config_request.name, "name");
        assert_eq!(config_request.description, "description");
        assert_eq!(config_request.route, "route");
        assert_eq!(config_request.parser, "parser");
        assert_eq!(config_request.handler, "handler");
    }

    #[test]
    fn test_config_request_from_json() {
        let data1 = r#"
            {
                "name": "name",
                "description": "description",
                "route": "route",
                "parser": "parser",
                "handler": "handler"
            }"#;
    
        let config_request1: ConfigRequest = serde_json::from_str(data1).unwrap();
        assert_eq!(config_request1.name, "name");
        assert_eq!(config_request1.description, "description");
        assert_eq!(config_request1.route, "route");
        assert_eq!(config_request1.parser, "parser");
        assert_eq!(config_request1.handler, "handler");

        let data_array1 = r#"
             [
                {
                    "name": "name1",
                    "description": "description1",
                    "route": "route1",
                    "parser": "parser1",
                    "handler": "handler1"
                },
                {
                    "name": "name2",
                    "description": "description2",
                    "route": "route2",
                    "parser": "parser2",
                    "handler": "handler2"
                },
                {
                    "name": "name3",
                    "description": "description3",
                    "route": "route3",
                    "parser": "parser3",
                    "handler": "handler3"
                }
             ]"#;
             
        let array: Vec<ConfigRequest> = serde_json::from_str(data_array1).unwrap();
        assert_eq!(array[0].name, "name1");
        assert_eq!(array[0].description, "description1");
        assert_eq!(array[0].route, "route1");
        assert_eq!(array[0].parser, "parser1");
        assert_eq!(array[0].handler, "handler1");
        assert_eq!(array[1].name, "name2");
        assert_eq!(array[1].description, "description2");
        assert_eq!(array[1].route, "route2");
        assert_eq!(array[1].parser, "parser2");
        assert_eq!(array[1].handler, "handler2");
        assert_eq!(array[2].name, "name3");
        assert_eq!(array[2].description, "description3");
        assert_eq!(array[2].route, "route3");
        assert_eq!(array[2].parser, "parser3");
        assert_eq!(array[2].handler, "handler3");
    }
}