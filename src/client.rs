use std::collections::HashMap;

use reqwest::{self, StatusCode};
use reqwest::header::Headers;
use serde_json;

use error::AuthyError;

#[derive(Debug)]
pub struct Client {
    api_url: String,
    api_key: String,
    reqwest: reqwest::Client,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    pub success: bool,
    pub message: String,
}

impl Client {
    pub fn new(api_url: &str, api_key: &str) -> Client {
        Client {
            api_url: api_url.into(), 
            api_key: api_key.into(),
            reqwest: reqwest::Client::new().expect("A reqwest client"),
        }
    }

    pub fn get(&self, prefix: &str, path: &str) -> Result<serde_json::Value, AuthyError> {
        self.request(self.reqwest.get(&self.url(prefix, path)))
    }

    pub fn post(&self, prefix: &str, path: &str, params: Option<HashMap<&str, &str>>) -> Result<serde_json::Value, AuthyError> {

        let url = self.url(prefix, path);
        match params {
            Some(p) => {
                self.request(self.reqwest.post(&url).form(&p))
            },
            None => {
                self.request(self.reqwest.post(&url))
            }
        }
    }

    fn url(&self, prefix: &str, path: &str) -> String {
        format!("{api_url}/{prefix}/json/{path}",
                api_url = self.api_url,
                prefix = prefix,
                path = path)
    }

    fn request(&self, request: reqwest::RequestBuilder) -> Result<serde_json::Value, AuthyError> {
        let mut headers = Headers::new();
        headers.set_raw("X-Authy-API-Key", vec![self.api_key.clone().into()]);

        let mut res = request.headers(headers).send()?;

        match res.status().clone() {
            StatusCode::Ok => Ok(res.json()?),
            StatusCode::ServiceUnavailable => Err(AuthyError::ServiceUnavailable),
            ref s => Err(AuthyError::from_status(s, res.json()?)),
        }
    }
}
