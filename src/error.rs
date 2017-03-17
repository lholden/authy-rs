use std::io;

use reqwest::{self, StatusCode};
use serde_json;

use client::Status;

#[derive(Debug)]
pub enum AuthyError {
    ServiceUnavailable,         // 503
    BadRequest(String),         // 400
    UnauthorizedToken(String),  // 401
    TooManyRequests(String),    // 429
    UnknownStatus(String),      // Other

    IoError(String),
    JsonError(String),
    RequestError(String),
    WrongContentType(String),
    MalformedResponse(String),
}

impl AuthyError {
    pub fn from_status(s: &reqwest::StatusCode, status: Status) -> AuthyError {
        match s {
            &StatusCode::BadRequest => AuthyError::BadRequest(status.message),
            &StatusCode::Unauthorized => AuthyError::UnauthorizedToken(status.message),
            &StatusCode::TooManyRequests => AuthyError::TooManyRequests(status.message),
            _ => AuthyError::UnknownStatus(status.message)
        }
    }
}

impl From<reqwest::Error> for AuthyError {
    fn from(e: reqwest::Error) -> Self {
        AuthyError::RequestError(e.to_string())
    }
}

impl From<serde_json::Error> for AuthyError {
    fn from(e: serde_json::Error) -> Self {
        AuthyError::JsonError(e.to_string())
    }
}

impl From<io::Error> for AuthyError {
    fn from(e: io::Error) -> Self {
        AuthyError::IoError(e.to_string())
    }
}
