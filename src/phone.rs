use std::fmt::{self, Display};

use serde_json;
use std::collections::HashMap;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "protected";

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneInfo {
    #[serde(rename = "type")]
    pub phone_type: String,
    pub provider: String,
    pub ported: bool,
}

#[derive(Debug)]
pub enum ContactType {
    SMS,
    Call,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneStart {
    pub is_ported: bool,
    pub is_cellphone: bool,
}

impl Display for ContactType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ContactType::SMS => write!(f, "sms"),
            ContactType::Call => write!(f, "call")
        } 
    }
}

pub fn info(client: &Client, country_code: u16, phone: &str, user_ip: Option<&str>) -> Result<(Status, PhoneInfo), AuthyError> {
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert("country_code", country_code.to_string());
    params.insert("phone_number", phone.into());
    if let Some(user_ip) = user_ip {
        params.insert("user_ip", user_ip.into());
    };

    let (status, res) = client.get(PREFIX, "phones/info", Some(params))?;

    let phone_info = serde_json::from_value(res)?;

    Ok((status, phone_info))
}

pub fn start(client: &Client, via: ContactType, country_code: u16, phone: &str, code_length: Option<u8>, locale: Option<&str>) -> Result<(Status, PhoneStart), AuthyError> {
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert("via", via.to_string());
    params.insert("country_code", country_code.to_string());
    params.insert("phone_number", phone.into());
    if let Some(code_length) = code_length {
        params.insert("code_length", code_length.to_string());
    };
    if let Some(locale) = locale {
        params.insert("locale", locale.into());
    };

    let (status, res) = client.post(PREFIX, "phones/verification/start", None, Some(params))?;
    let phone_verification = serde_json::from_value(res)?;

    Ok((status, phone_verification))
}

pub fn check(client: &Client, country_code: u16, phone: &str, code: &str) -> Result<Status, AuthyError> {
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert("country_code", country_code.to_string());
    params.insert("phone_number", phone.into());
    params.insert("verification_code", code.into());

    let (status, _) = client.get(PREFIX, "phones/verification/check", Some(params))?;

    Ok(status)
}
