use std::thread;
use std::io::Read;
use std::time::Duration;

use reqwest::{self, StatusCode, Method, Url};
use reqwest::header::Headers;
use serde_json::{self, Value};

use error::AuthyError;

#[derive(Debug)]
pub struct Client {
    pub retry_count: u8,
    pub retry_wait: u16,

    api_url: String,
    api_key: String,
    reqwest: reqwest::Client,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub success: bool,
    pub message: String,

    pub error_code: Option<String>,
}

impl Client {
    pub fn new(api_url: &str, api_key: &str) -> Client {
        Client {
            retry_count: 3,
            retry_wait: 250,
            api_url: api_url.into(), 
            api_key: api_key.into(),
            reqwest: reqwest::Client::new().expect("A reqwest client"),
        }
    }

    pub fn get(&self, prefix: &str, path: &str, url_params: Option<Vec<(String, String)>>) -> Result<(Status, Value), AuthyError> {
        self.request(Method::Get, self.url(prefix, path, url_params), None)
    }

    pub fn post(&self, prefix: &str, path: &str, url_params: Option<Vec<(String, String)>>, post_params: Option<Vec<(String, String)>>) -> Result<(Status, Value), AuthyError> {
        self.request(Method::Post, self.url(prefix, path, url_params), post_params)
    }

    fn url(&self, prefix: &str, path: &str, params: Option<Vec<(String, String)>>) -> Url {
        let base = format!("{api_url}/{prefix}/json/{path}", 
                           api_url = self.api_url,
                           prefix = prefix,
                           path = path);
        match params {
            Some(params) => Url::parse_with_params(&base, params),
            None => Url::parse(&base),
        }.expect("Url to be valid")
    }

    fn request(&self, method: Method, url: Url, params: Option<Vec<(String, String)>>) -> Result<(Status, Value), AuthyError> {
        let mut count = self.retry_count;
        loop {
            let url = url.clone();
            let mut headers = Headers::new();
            headers.set_raw("X-Authy-API-Key", vec![self.api_key.clone().into()]);
            let mut res = match params.clone() {
                Some(p) => self.reqwest.request(method.clone(), url).headers(headers).form(&p).send()?,
                None => self.reqwest.request(method.clone(), url).headers(headers).send()?,
            };

            let mut body = String::new();
            res.read_to_string(&mut body)?;
            println!("{}", body);

            // I wish could just check the content type but authy mixes json
            // and html content types when returning valid json.
            match serde_json::from_str::<Value>(&body) {
                Ok(value) => {
                    let status: Status = serde_json::from_str(&body)?;

                    match res.status() {
                        &StatusCode::Ok => return Ok((status, value)),
                        &StatusCode::BadRequest => return Err(AuthyError::BadRequest(status)),
                        &StatusCode::Unauthorized => return Err(AuthyError::UnauthorizedKey(status)),
                        &StatusCode::Forbidden => return Err(AuthyError::Forbidden(status)),
                        &StatusCode::TooManyRequests => return Err(AuthyError::TooManyRequests(status)),
                        &StatusCode::NotFound => return Err(AuthyError::UserNotFound(status)),
                        &StatusCode::InternalServerError => return Err(AuthyError::InternalServerError(status)),
                        s => panic!("Status code not covered in authy REST specification: {}", s),
                    };
                },
                Err(_) => {
                    match res.status() {
                        &StatusCode::ServiceUnavailable => {
                            count -= 1;
                            if count == 0 {
                                return Err(AuthyError::ServiceUnavailable);
                            }
                            else {
                                thread::sleep(Duration::from_millis(self.retry_wait.into()));
                                continue;
                            }
                        },
                        _ => return Err(AuthyError::InvalidServerResponse),
                    }
                },
            };
        }
    }
}
