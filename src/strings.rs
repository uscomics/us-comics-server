#![allow(dead_code)]
pub static AUTHENTICATION_NOT_CONFIGURED: usize = 0;
pub static AUTHORIZATION_NOT_CONFIGURED: usize = 1;
pub static UNAUTHORIZED: usize = 2;
pub static LOGIN_REQUIRED: usize = 3;
pub static INCORRECT_USER_NAME: usize = 4;
pub static INCORRECT_PASSWORD: usize = 5;
pub static LOGIN_SUCCESSFUL: usize = 6;
pub static FIRECRACKER_LISTENING_ON_PORT: usize = 7;
pub static FATAL_ERROR: usize = 8;
pub static ERROR: usize = 9;
pub static WARNING: usize = 10;
pub static INFO: usize = 11;
pub static DEBUG: usize = 12;
pub static TRACE: usize = 13;
pub static ALL: usize = 14;
pub static ENTERING: usize = 15;
pub static EXITING: usize = 16;
pub static SUCCESSFULLY_OPENED_FILE: usize = 17;
pub static SUCCESSFULLY_READ_FILE: usize = 18;
pub static COULD_NOT_OPEN_FILE: usize = 19;
pub static COULD_NOT_READ_FILE: usize = 20;
pub static NO_FILE_SPECIFIED: usize = 21;
pub static REQUEST_NOT_READ: usize = 22;
pub static REQUEST_NOT_FOUND: usize = 23;
pub static REQUEST_PROCESSED: usize = 24;

pub static COUNT: usize = 25;
