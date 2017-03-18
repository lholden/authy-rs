use std::fmt::{self, Display};

use serde_json;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "protected";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PhoneInfo {
    #[serde(rename = "type")]
    pub phone_type: String,
    pub provider: String,
    pub ported: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PhoneStart {
    pub is_ported: bool,
    pub is_cellphone: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContactType {
    SMS,
    Call,
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
    let mut params: Vec<(String, String)> = vec![];
    params.push(("country_code".into(), country_code.to_string()));
    params.push(("phone_number".into(), phone.into()));
    if let Some(user_ip) = user_ip {
        params.push(("user_ip".into(), user_ip.into()));
    };

    let (status, res) = client.get(PREFIX, "phones/info", Some(params))?;

    let phone_info = serde_json::from_value(res)?;

    Ok((status, phone_info))
}

pub fn start(client: &Client, via: ContactType, country_code: u16, phone: &str, code_length: Option<u8>, locale: Option<&str>) -> Result<(Status, PhoneStart), AuthyError> {
    let mut params: Vec<(String, String)> = vec![];
    params.push(("via".into(), via.to_string()));
    params.push(("country_code".into(), country_code.to_string()));
    params.push(("phone_number".into(), phone.into()));
    if let Some(code_length) = code_length {
        params.push(("code_length".into(), code_length.to_string()));
    };
    if let Some(locale) = locale {
        params.push(("locale".into(), locale.into()));
    };

    let (status, res) = client.post(PREFIX, "phones/verification/start", None, Some(params))?;
    let phone_verification = serde_json::from_value(res)?;

    Ok((status, phone_verification))
}

pub fn check(client: &Client, country_code: u16, phone: &str, code: &str) -> Result<Status, AuthyError> {
    let mut params: Vec<(String, String)> = vec![];
    params.push(("country_code".into(), country_code.to_string()));
    params.push(("phone_number".into(), phone.into()));
    params.push(("verification_code".into(), code.into()));

    let (status, _) = client.get(PREFIX, "phones/verification/check", Some(params))?;

    Ok(status)
}
