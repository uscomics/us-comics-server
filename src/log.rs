#![allow(dead_code)]
use crate::i18n;
use crate::strings;
use crate::server_status::ServerStatus;
use std::fmt;
use std::process;

///////////////////////////////////////////
// Valid logging levels
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Level {
    All = 0,
    Trace = 1,
    Debug = 2,
    Info = 3,
    Warning = 4,
    Error = 5,
    FatalError = 6,
    Off = 7,
}

///////////////////////////////////////////
// Logging macros
#[macro_export] macro_rules! style_text {
    ($level:expr, $arg:expr) => ({
        match $level {
            log::Level::All =>  { style($arg).white() },
            log::Level::Trace =>  { style($arg).white() },
            log::Level::Debug =>  { style($arg).cyan() },
            log::Level::Info =>  { style($arg).blue() },
            log::Level::Warning =>  { style($arg).yellow().bold()},
            log::Level::Error =>  { style($arg).red().bold() },
            log::Level::FatalError =>  { style($arg).red().bold().underlined() },
            log::Level::Off =>  { style($arg).black() }
        }
    })
}
#[macro_export] macro_rules! structured_log {
    ($log:expr, $level:expr, $($args:tt)+) => ({
        if $log.can($level) {
            let now: DateTime<Utc> = Utc::now();
            let file = file!();
            let line = line!();
            let prefix = $log.get_level_prefix($level).to_string();
            let colored_text = format!(
                "{}\"time\": \"{}-{}-{}T{}:{}:{}Z\", \"file\": \"{}\", \"line\": {}, {}, \"entry\": ",
                "{ ",
                now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second(),
                file,
                line,
                prefix
            );
            println!( "{}\"{:?}\"{}", style_text!($level, colored_text.to_string()), format_args!($($args)+ ), " }");
            format!("{}\"{:?}\"{}", colored_text.to_string(), format_args!($($args)+), " }")
        } else {
            format!("")
        }
    })
}

#[macro_export] macro_rules! structured_all {($log:expr, $($args:tt)+) => (structured_log!($log, log::Level::All, $($args)*))}
#[macro_export] macro_rules! structured_trace {($log:expr, $($args:tt)+) => (structured_log!($log, log::Level::Trace, $($args)*))}
#[macro_export] macro_rules! structured_debug {($log:expr, $($args:tt)+) => (structured_log!($log, log::Level::Debug, $($args)*))}
#[macro_export] macro_rules! structured_info {($log:expr, $($args:tt)+) => (structured_log!($log, log::Level::Info, $($args)*))}
#[macro_export] macro_rules! structured_warn {($log:expr, $($args:tt)+) => (structured_log!($log, log::Level::Warning, $($args)*))}
#[macro_export] macro_rules! structured_error {($log:expr, $($args:tt)+) => (structured_log!($log, log::Level::Error, $($args)*))}
#[macro_export] macro_rules! structured_fatal {($log:expr, $($args:tt)+) => (structured_log!($log, log::Level::FatalError, $($args)*))}

///////////////////////////////////////////
// Structured errors and messages.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StructuredMessage {
    Message {  message: String },
    EntryMessage {  message: String, verb: String, path: String },
    ExitMessage {  message: String, verb: String, path: String },
    FileMessage {  message: String, file: String },
}
impl fmt::Display for StructuredMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StructuredMessage::Message{  message } => write!(
                f,
                "{}{}{}{}",
                "{ ",
                "\"message\": \"", message,
                "\" }"
            ),
            StructuredMessage::EntryMessage{  message, verb, path  } => write!(
                f,
                "{}{}{}{}{}{}{}{}",
                "{ ",
                "\"message\" :\"", message,
                "\", \"verb\": \"", verb,
                "\", \"path\": \"", path,
                "\" }"
            ),
            StructuredMessage::ExitMessage{  message, verb, path } => write!(
                f,
                "{}{}{}{}{}{}{}{}",
                "{ ",
                "\"message\" :\"", message,
                "\", \"verb\": \"", verb,
                "\", \"path\": \"", path,
                "\" }"
            ),
            StructuredMessage::FileMessage{  message, file } => write!(
                f,
                "{}{}{}{}{}{}",
                "{ ",
                "\"message\" :\"", message,
                "\", \"file\": \"", file,
                "\" }"
            ),
        }
    }
}
#[allow(dead_code)]
pub fn build_message(message: &String) -> StructuredMessage{
    return StructuredMessage::Message{ message: message.to_string() };
}
#[allow(dead_code)]
pub fn build_entering_message(i18n: & i18n::I18n, verb: &String, path: &String) -> StructuredMessage {
    let entering = i18n.get(strings::ENTERING);
    return StructuredMessage::EntryMessage{ message: entering, verb: verb.to_string(), path: path.to_string() };
}
#[allow(dead_code)]
pub fn build_exiting_message(i18n: &i18n::I18n, verb: &String, path: &String) -> StructuredMessage{
    let exiting = i18n.get(strings::EXITING);
    return StructuredMessage::ExitMessage{ message: exiting, verb: verb.to_string(), path: path.to_string() };
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StructuredError {
    Error {  message: String, status_code: ServerStatus },
    FileError {  message: String, status_code: ServerStatus, file: String },
}
impl fmt::Display for StructuredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StructuredError::Error{  message, status_code } => write!(
                f,
                "{}{}{}{}{}{}",
                "{ ",
                "\"message\": \"", message,
                "\", \"status_code\": \"", status_code,
                "\" }"
            ),
            StructuredError::FileError{  message, status_code, file } => write!(
                f,
                "{}{}{}{}{}{}{}{}",
                "{ ",
                "\"message\": \"", message,
                "\", \"file\": \"", file,
                "\", \"status_code\": \"", status_code,
                "\" }"
            ),
        }
    }
}
#[allow(dead_code)]
pub fn build_error(message: &String, status_code: ServerStatus) -> StructuredError{
    return StructuredError::Error{ message: message.to_string(), status_code: status_code };
}
#[allow(dead_code)]
pub fn build_file_error(message: &String, file: &String, status_code: ServerStatus) -> StructuredError{
    return StructuredError::FileError{ message: message.to_string(), status_code: status_code, file: file.to_string() };
}

///////////////////////////////////////////
// Log.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Log{
    level: Level,
    prefix_fatal_error: String,
    prefix_error: String,
    prefix_warning: String,
    prefix_info: String,
    prefix_debug: String,
    prefix_trace: String,
    prefix_all: String,
}
impl Log {
    pub fn new(level: Level, i18n: &i18n::I18n) -> Log{
        let level = level;
        let prefix_fatal_error = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::FATAL_ERROR));
        let prefix_error = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::ERROR));
        let prefix_warning = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::WARNING));
        let prefix_info = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::INFO));
        let prefix_debug = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::DEBUG));
        let prefix_trace = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::TRACE));
        let prefix_all = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::ALL));
        return Log {
            level: level,
            prefix_fatal_error: prefix_fatal_error,
            prefix_error: prefix_error,
            prefix_warning: prefix_warning,
            prefix_info: prefix_info,
            prefix_debug: prefix_debug,
            prefix_trace: prefix_trace,
            prefix_all: prefix_all
        };
    }
    pub fn init(&mut self, level: Level, i18n: &i18n::I18n) {
        self.level = level;
        self.prefix_fatal_error = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::FATAL_ERROR));
        self.prefix_error = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::ERROR));
        self.prefix_warning = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::WARNING));
        self.prefix_info = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::INFO));
        self.prefix_debug = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::DEBUG));
        self.prefix_trace = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::TRACE));
        self.prefix_all = format!("{}, \"level\": \"{}\"", Log::prefix(), i18n.get(strings::ALL));
    }
    pub fn set(&mut self, level: Level) {
        self.level = level;
    }
    pub fn get_level(level_name: &str) -> Level {
        match level_name {
            "ALL" => { return Level::All; },
            "TRACE" => { return Level::Trace; },
            "DEBUG" => { return Level::Debug; },
            "INFO" => { return Level::Info; },
            "WARN" => { return Level::Warning; },
            "ERROR" => { return Level::Error; },
            "FATAL" => { return Level::FatalError; },
            _ => { return Level::Off; }
        }
    }
    pub fn can(&self, level: Level) -> bool {
        return level as u8 >= self.level as u8;
    }
    pub fn get_level_prefix(&self, level: Level) -> String {
        if Level::All == level { return format!("{}", self.prefix_all); }
        if Level::Trace == level { return format!("{}", self.prefix_trace); }
        if Level::Debug == level { return format!("{}", self.prefix_debug); }
        if Level::Info == level { return format!("{}", self.prefix_info); }
        if Level::Warning == level { return format!("{}", self.prefix_warning); }
        if Level::Error == level { return format!("{}", self.prefix_error); }
        if Level::FatalError == level { return format!("{}", self.prefix_fatal_error); }
        return "".to_string();
    }

    fn prefix() -> String {
        let hostname_option = hostname::get();
        let hostname = match hostname_option {
            Ok(hostname_result) => {
                match hostname_result.into_string(){
                    Ok(ref hn) => hn.to_string(),
                    Err(_e) => "".to_string()
                }
            },
            Err(_e) => "".to_string()
        };
        let id = process::id();
        return format!("\"host\": \"{}\", \"processId\": {}", hostname, id);
    }
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture
#[cfg(test)]
mod test {
    use crate::log;
    use crate::i18n;
    use crate::strings;
    use console::style;
    use chrono::prelude::*;

    #[test]
    fn test_get_level() {
        assert_eq!(log::Log::get_level("ALL"), log::Level::All);
        assert_eq!(log::Log::get_level("TRACE"), log::Level::Trace);
        assert_eq!(log::Log::get_level("DEBUG"), log::Level::Debug);
        assert_eq!(log::Log::get_level("INFO"), log::Level::Info);
        assert_eq!(log::Log::get_level("WARN"), log::Level::Warning);
        assert_eq!(log::Log::get_level("ERROR"), log::Level::Error);
        assert_eq!(log::Log::get_level("FATAL"), log::Level::FatalError);
        assert_eq!(log::Log::get_level("OFF"), log::Level::Off);
        assert_eq!(log::Log::get_level("JUNK"), log::Level::Off);
    }
    #[test]
    fn test_get_level_prefix() {
        let i18n = i18n::I18n::new(i18n::DEFAULT_LOCALE.to_string(), i18n::DEFAULT_PATH.to_string());
        let log = log::Log::new(log::Level::Debug, &i18n);
        assert_eq!(log.prefix_fatal_error, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::FATAL_ERROR)));
        assert_eq!(log.prefix_error, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::ERROR)));
        assert_eq!(log.prefix_warning, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::WARNING)));
        assert_eq!(log.prefix_info, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::INFO)));
        assert_eq!(log.prefix_debug, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::DEBUG)));
        assert_eq!(log.prefix_trace, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::TRACE)));
        assert_eq!(log.prefix_all, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::ALL)));
    }
    #[test]
    fn test_prefix() {
        let i18n = i18n::I18n::new(i18n::DEFAULT_LOCALE.to_string(), i18n::DEFAULT_PATH.to_string());
        let log = log::Log::new(log::Level::Debug, &i18n);
        assert_eq!( log.level, log::Level::Debug);
        assert_eq!( log.prefix_fatal_error, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::FATAL_ERROR)) );
        assert_eq!( log.prefix_error, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::ERROR)) );
        assert_eq!( log.prefix_warning, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::WARNING)) );
        assert_eq!( log.prefix_info, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::INFO)) );
        assert_eq!( log.prefix_debug, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::DEBUG)) );
        assert_eq!( log.prefix_trace, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::TRACE)) );
        assert_eq!( log.prefix_all, format!("{}, \"level\": \"{}\"", log::Log::prefix(), i18n.get(strings::ALL)) );
    }
    #[test]
    fn test_can_and_set() {
        let i18n = i18n::I18n::new(i18n::DEFAULT_LOCALE.to_string(), i18n::DEFAULT_PATH.to_string());
        let mut log = log::Log::new(log::Level::All, &i18n);
        assert_eq!(log.can( log::Level::FatalError), true);
        assert_eq!(log.can( log::Level::Error), true);
        assert_eq!(log.can( log::Level::Warning), true);
        assert_eq!(log.can( log::Level::Info), true);
        assert_eq!(log.can( log::Level::Debug), true);
        assert_eq!(log.can( log::Level::Trace), true);
        assert_eq!(log.can( log::Level::All), true);
        log.set(log::Level::Trace);
        assert_eq!(log.can( log::Level::FatalError), true);
        assert_eq!(log.can( log::Level::Error), true);
        assert_eq!(log.can( log::Level::Warning), true);
        assert_eq!(log.can( log::Level::Info), true);
        assert_eq!(log.can( log::Level::Debug), true);
        assert_eq!(log.can( log::Level::Trace), true);
        assert_eq!(log.can( log::Level::All), false);
        log.set(log::Level::Debug);
        assert_eq!(log.can( log::Level::FatalError), true);
        assert_eq!(log.can( log::Level::Error), true);
        assert_eq!(log.can( log::Level::Warning), true);
        assert_eq!(log.can( log::Level::Info), true);
        assert_eq!(log.can( log::Level::Debug), true);
        assert_eq!(log.can( log::Level::Trace), false);
        assert_eq!(log.can( log::Level::All), false);
        log.set(log::Level::Info);
        assert_eq!(log.can( log::Level::FatalError), true);
        assert_eq!(log.can( log::Level::Error), true);
        assert_eq!(log.can( log::Level::Warning), true);
        assert_eq!(log.can( log::Level::Info), true);
        assert_eq!(log.can( log::Level::Debug), false);
        assert_eq!(log.can( log::Level::Trace), false);
        assert_eq!(log.can( log::Level::All), false);
        log.set(log::Level::Warning);
        assert_eq!(log.can( log::Level::FatalError), true);
        assert_eq!(log.can( log::Level::Error), true);
        assert_eq!(log.can( log::Level::Warning), true);
        assert_eq!(log.can( log::Level::Info), false);
        assert_eq!(log.can( log::Level::Debug), false);
        assert_eq!(log.can( log::Level::Trace), false);
        assert_eq!(log.can( log::Level::All), false);
        log.set(log::Level::Error);
        assert_eq!(log.can( log::Level::FatalError), true);
        assert_eq!(log.can( log::Level::Error), true);
        assert_eq!(log.can( log::Level::Warning), false);
        assert_eq!(log.can( log::Level::Info), false);
        assert_eq!(log.can( log::Level::Debug), false);
        assert_eq!(log.can( log::Level::Trace), false);
        assert_eq!(log.can( log::Level::All), false);
        log.set(log::Level::FatalError);
        assert_eq!(log.can( log::Level::FatalError), true);
        assert_eq!(log.can( log::Level::Error), false);
        assert_eq!(log.can( log::Level::Warning), false);
        assert_eq!(log.can( log::Level::Info), false);
        assert_eq!(log.can( log::Level::Debug), false);
        assert_eq!(log.can( log::Level::Trace), false);
        assert_eq!(log.can( log::Level::All), false);
        log.set(log::Level::Off);
        assert_eq!(log.can( log::Level::FatalError), false);
        assert_eq!(log.can( log::Level::Error), false);
        assert_eq!(log.can( log::Level::Warning), false);
        assert_eq!(log.can( log::Level::Info), false);
        assert_eq!(log.can( log::Level::Debug), false);
        assert_eq!(log.can( log::Level::Trace), false);
        assert_eq!(log.can( log::Level::All), false);
    }

    #[test]
    fn test_macros() {
        let i18n = i18n::I18n::new(i18n::DEFAULT_LOCALE.to_string(), i18n::DEFAULT_PATH.to_string());
        let mut log = log::Log::new(log::Level::All, &i18n);
        assert_eq!(structured_log!(log, log::Level::FatalError, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Error, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Warning, "WARNING" ).ends_with( "\"WARNING\", \"entry\": \"WARNING\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Info, "INFO" ).ends_with( "\"INFO\", \"entry\": \"INFO\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Debug, "DEBUG" ).ends_with( "\"DEBUG\", \"entry\": \"DEBUG\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Trace, "TRACE" ).ends_with( "\"TRACE\", \"entry\": \"TRACE\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::All, "ALL" ).ends_with( "\"ALL\", \"entry\": \"ALL\" }" ), true);
        log.set(log::Level::Trace);
        assert_eq!(structured_log!(log, log::Level::FatalError, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Error, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Warning, "WARNING" ).ends_with( "\"WARNING\", \"entry\": \"WARNING\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Info, "INFO" ).ends_with( "\"INFO\", \"entry\": \"INFO\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Debug, "DEBUG" ).ends_with( "\"DEBUG\", \"entry\": \"DEBUG\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Trace, "TRACE" ).ends_with( "\"TRACE\", \"entry\": \"TRACE\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::All, "ALL" ), "".to_string());
        log.set(log::Level::Debug);
        assert_eq!(structured_log!(log, log::Level::FatalError, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Error, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Warning, "WARNING" ).ends_with( "\"WARNING\", \"entry\": \"WARNING\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Info, "INFO" ).ends_with( "\"INFO\", \"entry\": \"INFO\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Debug, "DEBUG" ).ends_with( "\"DEBUG\", \"entry\": \"DEBUG\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Trace, "TRACE" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::All, "ALL" ), "".to_string());
        log.set(log::Level::Info);
        assert_eq!(structured_log!(log, log::Level::FatalError, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Error, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Warning, "WARNING" ).ends_with( "\"WARNING\", \"entry\": \"WARNING\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Info, "INFO" ).ends_with( "\"INFO\", \"entry\": \"INFO\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Debug, "DEBUG" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Trace, "TRACE" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::All, "ALL" ), "".to_string());
        log.set(log::Level::Warning);
        assert_eq!(structured_log!(log, log::Level::FatalError, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Error, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Warning, "WARNING" ).ends_with( "\"WARNING\", \"entry\": \"WARNING\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Info, "INFO" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Debug, "DEBUG" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Trace, "TRACE" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::All, "ALL" ), "".to_string());
        log.set(log::Level::Error);
        assert_eq!(structured_log!(log, log::Level::FatalError, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Error, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Warning, "WARNING" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Info, "INFO" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Debug, "DEBUG" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Trace, "TRACE" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::All, "ALL" ), "".to_string());
        log.set(log::Level::FatalError);
        assert_eq!(structured_log!(log, log::Level::FatalError, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_log!(log, log::Level::Error, "ERROR" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Warning, "WARNING" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Info, "INFO" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Debug, "DEBUG" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Trace, "TRACE" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::All, "ALL" ), "".to_string());
        log.set(log::Level::Off);
        assert_eq!(structured_log!(log, log::Level::FatalError, "FATAL_ERROR" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Error, "ERROR" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Warning, "WARNING" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Info, "INFO" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Debug, "DEBUG" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::Trace, "TRACE" ), "".to_string());
        assert_eq!(structured_log!(log, log::Level::All, "ALL" ), "".to_string());
        log.set(log::Level::All);
        let log_text = structured_log!(log, log::Level::All, "Log Text {}.{}.{}.{}.{}", "one", "two", "three", "four", "five");
        assert_eq!( log_text.ends_with( "\"ALL\", \"entry\": \"Log Text one.two.three.four.five\" }" ), true);

        log.init(log::Level::All, &i18n);
        assert_eq!(structured_fatal!( log, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_error!( log, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_warn!( log, "WARNING" ).ends_with( "\"WARNING\", \"entry\": \"WARNING\" }" ), true);
        assert_eq!(structured_info!( log, "INFO" ).ends_with( "\"INFO\", \"entry\": \"INFO\" }" ), true);
        assert_eq!(structured_debug!( log, "DEBUG" ).ends_with( "\"DEBUG\", \"entry\": \"DEBUG\" }" ), true);
        assert_eq!(structured_trace!( log, "TRACE" ).ends_with( "\"TRACE\", \"entry\": \"TRACE\" }" ), true);
        assert_eq!(structured_all!( log, "ALL" ).ends_with( "\"ALL\", \"entry\": \"ALL\" }" ), true);
        log.set(log::Level::Trace);
        assert_eq!(structured_fatal!( log, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_error!( log, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_warn!( log, "WARNING" ).ends_with( "\"WARNING\", \"entry\": \"WARNING\" }" ), true);
        assert_eq!(structured_info!( log, "INFO" ).ends_with( "\"INFO\", \"entry\": \"INFO\" }" ), true);
        assert_eq!(structured_debug!( log, "DEBUG" ).ends_with( "\"DEBUG\", \"entry\": \"DEBUG\" }" ), true);
        assert_eq!(structured_trace!( log, "TRACE" ).ends_with( "\"TRACE\", \"entry\": \"TRACE\" }" ), true);
        assert_eq!(structured_all!( log, "ALL" ), "".to_string());
        log.set(log::Level::Debug);
        assert_eq!(structured_fatal!( log, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_error!( log, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_warn!( log, "WARNING" ).ends_with( "\"WARNING\", \"entry\": \"WARNING\" }" ), true);
        assert_eq!(structured_info!( log, "INFO" ).ends_with( "\"INFO\", \"entry\": \"INFO\" }" ), true);
        assert_eq!(structured_debug!( log, "DEBUG" ).ends_with( "\"DEBUG\", \"entry\": \"DEBUG\" }" ), true);
        assert_eq!(structured_trace!( log, "TRACE" ), "".to_string());
        assert_eq!(structured_all!( log, "ALL" ), "".to_string());
        log.set(log::Level::Info);
        assert_eq!(structured_fatal!( log, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_error!( log, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_warn!( log, "WARNING" ).ends_with( "\"WARNING\", \"entry\": \"WARNING\" }" ), true);
        assert_eq!(structured_info!( log, "INFO" ).ends_with( "\"INFO\", \"entry\": \"INFO\" }" ), true);
        assert_eq!(structured_debug!( log, "DEBUG" ), "".to_string());
        assert_eq!(structured_trace!( log, "TRACE" ), "".to_string());
        assert_eq!(structured_all!( log, "ALL" ), "".to_string());
        log.set(log::Level::Warning);
        assert_eq!(structured_fatal!( log, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_error!( log, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_warn!( log, "WARNING" ).ends_with( "\"WARNING\", \"entry\": \"WARNING\" }" ), true);
        assert_eq!(structured_info!( log, "INFO" ), "".to_string());
        assert_eq!(structured_debug!( log, "DEBUG" ), "".to_string());
        assert_eq!(structured_trace!( log, "TRACE" ), "".to_string());
        assert_eq!(structured_all!( log, "ALL" ), "".to_string());
        log.set(log::Level::Error);
        assert_eq!(structured_fatal!( log, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_error!( log, "ERROR" ).ends_with( "\"ERROR\", \"entry\": \"ERROR\" }" ), true);
        assert_eq!(structured_warn!( log, "WARNING" ), "".to_string());
        assert_eq!(structured_info!( log, "INFO" ), "".to_string());
        assert_eq!(structured_debug!( log, "DEBUG" ), "".to_string());
        assert_eq!(structured_trace!( log, "TRACE" ), "".to_string());
        assert_eq!(structured_all!( log, "ALL" ), "".to_string());
        log.set(log::Level::FatalError);
        assert_eq!(structured_fatal!( log, "FATAL_ERROR" ).ends_with( "\"FATAL ERROR\", \"entry\": \"FATAL_ERROR\" }" ), true);
        assert_eq!(structured_error!( log, "ERROR" ), "".to_string());
        assert_eq!(structured_warn!( log, "WARNING" ), "".to_string());
        assert_eq!(structured_info!( log, "INFO" ), "".to_string());
        assert_eq!(structured_debug!( log, "DEBUG" ), "".to_string());
        assert_eq!(structured_trace!( log, "TRACE" ), "".to_string());
        assert_eq!(structured_all!( log, "ALL" ), "".to_string());
        log.set(log::Level::Off);
        assert_eq!(structured_fatal!( log, "FATAL_ERROR" ), "".to_string());
        assert_eq!(structured_error!( log, "ERROR" ), "".to_string());
        assert_eq!(structured_warn!( log, "WARNING" ), "".to_string());
        assert_eq!(structured_info!( log, "INFO" ), "".to_string());
        assert_eq!(structured_debug!( log, "DEBUG" ), "".to_string());
        assert_eq!(structured_trace!( log, "TRACE" ), "".to_string());
        assert_eq!(structured_all!( log, "ALL" ), "".to_string());
        log.set(log::Level::All);
        let log_text = structured_all!(log, "Log Text {}.{}.{}.{}.{}", "one", "two", "three", "four", "five");
        assert_eq!( log_text.ends_with( "\"ALL\", \"entry\": \"Log Text one.two.three.four.five\" }" ), true);
        let log_text = structured_trace!(log, "Log Text {}.{}.{}.{}.{}", "one", "two", "three", "four", "five");
        assert_eq!( log_text.ends_with( "\"TRACE\", \"entry\": \"Log Text one.two.three.four.five\" }" ), true);
        let log_text = structured_debug!(log, "Log Text {}.{}.{}.{}.{}", "one", "two", "three", "four", "five");
        assert_eq!( log_text.ends_with( "\"DEBUG\", \"entry\": \"Log Text one.two.three.four.five\" }" ), true);
        let log_text = structured_info!(log, "Log Text {}.{}.{}.{}.{}", "one", "two", "three", "four", "five");
        assert_eq!( log_text.ends_with( "\"INFO\", \"entry\": \"Log Text one.two.three.four.five\" }" ), true);
        let log_text = structured_warn!(log, "Log Text {}.{}.{}.{}.{}", "one", "two", "three", "four", "five");
        assert_eq!( log_text.ends_with( "\"WARNING\", \"entry\": \"Log Text one.two.three.four.five\" }" ), true);
        let log_text = structured_error!(log, "Log Text {}.{}.{}.{}.{}", "one", "two", "three", "four", "five");
        assert_eq!( log_text.ends_with( "\"ERROR\", \"entry\": \"Log Text one.two.three.four.five\" }" ), true);
        let log_text = structured_fatal!(log, "Log Text {}.{}.{}.{}.{}", "one", "two", "three", "four", "five");
        assert_eq!( log_text.ends_with( "\"FATAL ERROR\", \"entry\": \"Log Text one.two.three.four.five\" }" ), true);
    }
}