use std::io::{self, BufRead};
use std::io::prelude::*;
use std::fs::File;

pub static DEFAULT_LOCALE: &str = "en-US";
pub static DEFAULT_PATH: &str = "./src/config/i18n/strings-en-US.txt";

#[derive(Debug, PartialEq, Eq)]
pub struct I18n{
    locale: String,
    strings: Vec<String>,
    path: String
}

impl I18n {
    pub fn new(locale: String, path: String) -> I18n {
        let vec = vec![];
        let mut i18n = I18n {
            locale: locale.to_string(),
            strings: vec,
            path: path
        };
        i18n.set_locale(locale.to_string());
        return i18n;
    }
    pub fn init(&mut self, locale: String, path: String) {
        self.path = path;
        self.set_locale(locale.to_string());
    }
    pub fn set_locale(&mut self, locale: String) {
        let mut file = File::open(self.path.to_string()).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        contents = contents.to_owned();
        let cursor = io::Cursor::new(contents);
        let mut lines_iter = cursor.lines().map(|l| l.unwrap());
        let mut optional = lines_iter.next();

        self.locale = locale;
        self.strings.clear();
        while let Some(text) = optional {
            self.strings.push(text);
            optional = lines_iter.next();
        }
    }
    pub fn get(&self, string_id: usize) -> String{
        if self.strings.len() <= string_id { return "".to_string(); }
        return self.strings[string_id].to_string();
    }
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use super::*;
    use crate::strings;

    #[test]
    fn test_new() {
        let i18n = I18n::new( "en-US".to_string(), "./src/config/i18n/strings-en-US.txt".to_string());
        assert_eq!(i18n.locale, "en-US");
        assert_eq!(i18n.path, "./src/config/i18n/strings-en-US.txt");
        assert_eq!(i18n.strings.len(), strings::COUNT);
    }
    #[test]
    fn test_get() {
        let i18n = I18n::new( "en-US".to_string(), "./src/config/i18n/strings-en-US.txt".to_string());
        assert_eq!(i18n.get(strings::AUTHENTICATION_NOT_CONFIGURED), "Authentication not configured.");
        assert_eq!(i18n.get(strings::AUTHORIZATION_NOT_CONFIGURED), "Authorization not configured.");
        assert_eq!(i18n.get(strings::UNAUTHORIZED), "Unauthorized.");
        assert_eq!(i18n.get(strings::LOGIN_REQUIRED), "Login required.");
        assert_eq!(i18n.get(strings::INCORRECT_USER_NAME), "Incorrect username:");
        assert_eq!(i18n.get(strings::INCORRECT_PASSWORD), "Incorrect password.");
        assert_eq!(i18n.get(strings::LOGIN_SUCCESSFUL), "Login successful.");
        assert_eq!(i18n.get(strings::LISTENING_ON_PORT), "Listening on port");
        assert_eq!(i18n.get(strings::FATAL_ERROR), "FATAL ERROR");
        assert_eq!(i18n.get(strings::ERROR), "ERROR");
        assert_eq!(i18n.get(strings::WARNING), "WARNING");
        assert_eq!(i18n.get(strings::INFO), "INFO");
        assert_eq!(i18n.get(strings::DEBUG), "DEBUG");
        assert_eq!(i18n.get(strings::TRACE), "TRACE");
        assert_eq!(i18n.get(strings::ALL), "ALL");
        assert_eq!(i18n.get(strings::INVALID_HEADER_DATA_IN_CONFIG), "Invalid header data found in config file.");
    }
}