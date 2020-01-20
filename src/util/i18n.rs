#![allow(dead_code)]
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::fs::File;

pub static DEFAULT_LOCALE: &str = "en-US";
pub static DEFAULT_PATH: &str = "./config/i18n/strings-en-US.txt";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct I18n{
    pub locale: String,
    pub strings: Vec<String>,
    pub path: String
}

#[allow(dead_code)]
impl I18n {
    pub fn new(locale: &str, path: &str) -> I18n {
        let vec = vec![];
        let mut i18n = I18n {
            locale: locale.to_string(),
            strings: vec,
            path: path.to_string()
        };
        i18n.set_locale(locale);
        return i18n;
    }
    pub fn init(&mut self, locale: &str, path: &str) {
        self.path = path.to_string();
        self.set_locale(locale);
    }
    pub fn set_locale(&mut self, locale: &str) {
        let mut file = File::open(self.path.to_string()).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        contents = contents.to_owned();
        let cursor = io::Cursor::new(contents);
        let mut lines_iter = cursor.lines().map(|l| l.unwrap());
        let mut optional = lines_iter.next();

        self.locale = locale.to_string();
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
    pub fn get_from_locale(string_id: usize, locale: &str, path: &str) -> String{
        let i18n = I18n::new(locale, path);
        i18n.get(string_id)
    }
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::strings;

    #[test]
    fn test_new() {
        let i18n = I18n::new( "en-US", "./config/i18n/strings-en-US.txt");
        assert_eq!(i18n.locale, "en-US");
        assert_eq!(i18n.path, "./config/i18n/strings-en-US.txt");
        assert_eq!(i18n.strings.len(), strings::COUNT);
    }
    #[test]
    fn test_get() {
        let i18n = I18n::new( "en-US", "./config/i18n/strings-en-US.txt");
        assert_eq!(i18n.get(strings::AUTHENTICATION_NOT_CONFIGURED), "Authentication not configured.");
        assert_eq!(i18n.get(strings::AUTHORIZATION_NOT_CONFIGURED), "Authorization not configured.");
        assert_eq!(i18n.get(strings::UNAUTHORIZED), "Unauthorized.");
        assert_eq!(i18n.get(strings::LOGIN_REQUIRED), "Login required.");
        assert_eq!(i18n.get(strings::INCORRECT_USER_NAME), "Incorrect username:");
        assert_eq!(i18n.get(strings::INCORRECT_PASSWORD), "Incorrect password.");
        assert_eq!(i18n.get(strings::LOGIN_SUCCESSFUL), "Login successful.");
        assert_eq!(i18n.get(strings::LISTENING_ON_PORT), "Listening on");
        assert_eq!(i18n.get(strings::FATAL_ERROR), "FATAL ERROR");
        assert_eq!(i18n.get(strings::ERROR), "ERROR");
        assert_eq!(i18n.get(strings::WARNING), "WARNING");
        assert_eq!(i18n.get(strings::INFO), "INFO");
        assert_eq!(i18n.get(strings::DEBUG), "DEBUG");
        assert_eq!(i18n.get(strings::TRACE), "TRACE");
        assert_eq!(i18n.get(strings::ALL), "ALL");
        assert_eq!(i18n.get(strings::ENTERING), "-> Entering");
        assert_eq!(i18n.get(strings::EXITING), "<- Exiting");
        assert_eq!(i18n.get(strings::SUCCESSFULLY_OPENED_FILE), "Successfully opened file.");
        assert_eq!(i18n.get(strings::SUCCESSFULLY_READ_FILE), "Successfully read file.");
        assert_eq!(i18n.get(strings::COULD_NOT_OPEN_FILE), "Could not open file.");
        assert_eq!(i18n.get(strings::COULD_NOT_READ_FILE), "Could not read file.");
        assert_eq!(i18n.get(strings::NO_FILE_SPECIFIED), "No file specified.");
        assert_eq!(i18n.get(strings::REQUEST_NOT_READ), "Request not read.");
        assert_eq!(i18n.get(strings::REQUEST_NOT_FOUND), "Request not found.");
        assert_eq!(i18n.get(strings::REQUEST_PROCESSED), "Request Processed.");
    }
}