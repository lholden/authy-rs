use std::error;
use std::io;
use std::fmt;

use reqwest;
use serde_json;

use client::Status;

/// The error type used by this library.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AuthyError {
    /// There was an error with the request.
    BadRequest(Status), // 400

    /// Either the API key or the verification token was invalid.
    UnauthorizedKey(Status), // 401

    /// This account does not have access to the requested service.
    Forbidden(Status), // 403

    /// The authy user could not be found
    UserNotFound(Status), // 404

    /// You have reached the API usage limit.
    TooManyRequests(Status), // 429

    /// There was an internal server error.
    InternalServerError(Status), // 500

    /// The authy service was unavailable. Only returned after the configured `retry_count`.
    ServiceUnavailable, // 503

    /// There was an IO error.
    IoError(String),

    /// There was an error deserializing a json object.
    JsonParseError(String),

    /// We made a request the server didn't like.
    RequestError(String),

    /// The server gave an invalid response.
    InvalidServerResponse,
}

impl error::Error for AuthyError {
    fn description(&self) -> &str {
        use AuthyError::*;
        match *self {
            BadRequest(_) => "400 bad request",
            UnauthorizedKey(_) => "401 unauthorized",
            Forbidden(_) => "403 forbidden",
            UserNotFound(_) => "404 not found",
            TooManyRequests(_) => "429 too many requests",
            InternalServerError(_) => "500 internal server error",
            ServiceUnavailable => "503 service unavailable",
            IoError(_) => "IO error",
            JsonParseError(_) => "JSON parse error",
            RequestError(_) => "Request error",
            InvalidServerResponse => "Invalid server response",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for AuthyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AuthyError::*;

        match *self {
            BadRequest(ref s) => write!(f, "Bad Request: {}", s.message),
            UnauthorizedKey(ref s) => write!(f, "Unauthorized API Key: {}", s.message),
            Forbidden(ref s) => write!(f, "Forbidden: {}", s.message),
            UserNotFound(ref s) => write!(f, "User Not Found: {}", s.message),
            TooManyRequests(ref s) => write!(f, "Too Many Requests: {}", s.message),
            InternalServerError(ref s) => write!(f, "Internal Server Error: {}", s.message),
            ServiceUnavailable => write!(f, "Service Unavailable reported by authy service"),
            IoError(ref s) => write!(f, "IO Error: {}", s),
            JsonParseError(ref s) => write!(f, "Json parsing error: {}", s),
            RequestError(ref s) => write!(f, "Request error: {}", s),
            InvalidServerResponse => write!(f, "Server returned an invalid response"),
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
        AuthyError::JsonParseError(e.to_string())
    }
}

impl From<io::Error> for AuthyError {
    fn from(e: io::Error) -> Self {
        AuthyError::IoError(e.to_string())
    }
}
