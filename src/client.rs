use std::thread;
use std::io::Read;
use std::collections::HashMap;
use std::time::Duration;

use reqwest::{self, StatusCode, Method};
use reqwest::header::Headers;
use serde_json;

use error::AuthyError;

#[derive(Debug)]
pub struct Client {
    pub retry_count: u8,
    pub retry_wait: u16,

    api_url: String,
    api_key: String,
    reqwest: reqwest::Client,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Status {
    pub success: bool,
    pub message: String,

    pub error_code: Option<String>,
}

impl Client {
    pub fn new(api_url: &str, api_key: &str) -> Client {
        Client {
            retry_count: 3,
            retry_wait: 200,
            api_url: api_url.into(), 
            api_key: api_key.into(),
            reqwest: reqwest::Client::new().expect("A reqwest client"),
        }
    }

    pub fn get(&self, prefix: &str, path: &str) -> Result<serde_json::Value, AuthyError> {
        self.request(Method::Get, &self.url(prefix, path), None)
    }

    pub fn post(&self, prefix: &str, path: &str, params: Option<HashMap<&str, &str>>) -> Result<serde_json::Value, AuthyError> {
        self.request(Method::Post, &self.url(prefix, path), params)
    }

    fn url(&self, prefix: &str, path: &str) -> String {
        format!("{api_url}/{prefix}/json/{path}",
                api_url = self.api_url,
                prefix = prefix,
                path = path)
    }

    fn request(&self, method: Method, url: &str, params: Option<HashMap<&str, &str>>) -> Result<serde_json::Value, AuthyError> {
        let mut count = self.retry_count;
        loop {
            let mut headers = Headers::new();
            headers.set_raw("X-Authy-API-Key", vec![self.api_key.clone().into()]);
            let mut res = match params.clone() {
                Some(p) => self.reqwest.request(method.clone(), url).headers(headers).form(&p).send()?,
                None => self.reqwest.request(method.clone(), url).headers(headers).send()?,
            };

            let mut body = String::new();
            res.read_to_string(&mut body)?;

            match res.status().clone() {
                StatusCode::Ok => return Ok(serde_json::from_str(&body)?),
                StatusCode::ServiceUnavailable => {
                    if count == 0 {
                        return Err(AuthyError::ServiceUnavailable);
                    }
                    else {
                        count -= 1;
                        thread::sleep(Duration::from_millis(self.retry_wait.into()));
                        continue
                    }
                },
                StatusCode::TooManyRequests => return Err(AuthyError::TooManyRequests(serde_json::from_str(&body)?)),
                StatusCode::Unauthorized => return Err(AuthyError::UnauthorizedKey(serde_json::from_str(&body)?)),
                StatusCode::BadRequest => return Err(AuthyError::BadRequest(serde_json::from_str(&body)?)),
                s => return Err(AuthyError::UnexpectedStatus(s)),
            }
        }
    }
}
