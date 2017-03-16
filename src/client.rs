use std::collections::HashMap;

use reqwest;
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
        let url = format!("{api_url}/{prefix}/json/{path}",
                          api_url = self.api_url,
                          prefix = prefix,
                          path = path);

        let mut headers = Headers::new();
        headers.set_raw("X-Authy-API-Key", vec![self.api_key.clone().into()]);

        let mut res = self.reqwest.get(&url).headers(headers).send()?;

        let body = res.json::<serde_json::Value>()?;

        if res.status().is_success() {
            Ok(body)
        }
        else {
            Err(AuthyError::from_status(res.status(), serde_json::from_value(body)?))
        }
    }

    pub fn post(&self, prefix: &str, path: &str, params: Option<HashMap<&str, &str>>) -> Result<serde_json::Value, AuthyError> {
        let url = format!("{api_url}/{prefix}/json/{path}?api_key",
                          api_url = self.api_url,
                          prefix = prefix,
                          path = path);

        let mut headers = Headers::new();
        headers.set_raw("X-Authy-API-Key", vec![self.api_key.clone().into()]);

        let mut res = match params {
            Some(p) => {
                self.reqwest.post(&url).headers(headers).form(&p).send()?
            },
            None => {
                self.reqwest.post(&url).headers(headers).send()?
            }
        };

        let body = res.json::<serde_json::Value>()?;

        if res.status().is_success() {
            Ok(body)
        }
        else {
            Err(AuthyError::from_status(res.status(), serde_json::from_value(body)?))
        }
    }
}
