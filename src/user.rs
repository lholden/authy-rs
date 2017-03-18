use std::fmt::{self, Display};

use serde_json;
use std::collections::HashMap;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "protected";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserStatus {
    #[serde(rename = "authy_id")]
    pub id: u32,
    pub confirmed: bool,
    pub registered: bool,
    pub account_disabled: bool,

    pub country_code: u16,
    pub phone_number: String,
    pub has_hard_token: bool,

    pub devices: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Phone {
    pub cellphone: String,

    pub device: Option<String>,
    pub ignored: Option<bool>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ActivityType {
    PasswordReset,
    Banned,
    Unbanned,
    CookieLogin,
}

impl Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActivityType::PasswordReset => write!(f, "password_reset"),
            ActivityType::Banned => write!(f, "banned"),
            ActivityType::Unbanned => write!(f, "unbanned"),
            ActivityType::CookieLogin => write!(f, "cookie_login"),
        }
    }
}

pub fn new(client: &Client, email: &str, country_code: u16, phone: &str, send_install: bool) -> Result<(Status, User), AuthyError> {
    let mut params: Vec<(String, String)> = vec![];
    params.push(("user[email]".into(), email.into()));
    params.push(("user[cellphone]".into(), phone.into()));
    params.push(("user[country_code]".into(), country_code.to_string()));
    if send_install {
        params.push(("send_install_link_via_sms".into(), "true".into()));
    }

    let (status, res) = client.post(PREFIX, "users/new", None, Some(params))?;

    let user = serde_json::from_value(res["user"].clone())?;

    Ok((status, user))
}

pub fn delete(client: &Client, id: u32) -> Result<Status, AuthyError> {
    let (status, _) = client.post(PREFIX, &format!("users/{}/delete", id), None, None)?;

    Ok(status)
}

pub fn status(client: &Client, id: u32) -> Result<(Status, UserStatus), AuthyError> {
    let (status, res) = client.get(PREFIX, &format!("users/{}/status", id), None)?;

    let user_status = serde_json::from_value(res["status"].clone())?;

    Ok((status, user_status))
}

pub fn verify(client: &Client, id: u32, token: &str) -> Result<Status, AuthyError> {
    let (status, _) = client.get(PREFIX, &format!("verify/{token}/{id}", token = token, id = id), None)?;

    Ok(status)
}

fn phone(client: &Client, kind: &str, id: u32, force: bool, action: Option<&str>, action_message: Option<&str>) -> Result<(Status, Phone), AuthyError> {
    let mut params: Vec<(String, String)> = vec![];
    params.push(("force".into(), force.to_string()));
    if let Some(action) = action {
        params.push(("action".into(), action.into()));
    }
    if let Some(action_message) = action_message {
        params.push(("action_message".into(), action_message.into()));
    }

    let (status, res) = client.get(PREFIX, &format!("{}/{}", kind, id), Some(params))?;

    let sms = serde_json::from_value(res)?;

    Ok((status, sms))
}

pub fn sms(client: &Client, id: u32, force: bool, action: Option<&str>, action_message: Option<&str>) -> Result<(Status, Phone), AuthyError> {
    phone(client, "sms", id, force, action, action_message)
}

pub fn call(client: &Client, id: u32, force: bool, action: Option<&str>, action_message: Option<&str>) -> Result<(Status, Phone), AuthyError> {
    phone(client, "call", id, force, action, action_message)
}

// TODO: The REST server returns a 500 error for this. Figure out why so it can
// be re-enabled.
pub fn register_activity(client: &Client, id: u32, data: Option<&HashMap<&str, String>>, activity_type: ActivityType, user_ip: &str) -> Result<Status, AuthyError> {
    let mut params: Vec<(String, String)> = vec![];
    params.push(("type".into(), activity_type.to_string()));
    params.push(("user_ip".into(), user_ip.into()));

    if let Some(data) = data {
        for (k, v) in data {
            params.push((format!("data[{}]", k), v.clone()));
        }
    }

    let (status, _) = client.post(PREFIX, &format!("users/{}/register_activity", id), None, Some(params))?;

    Ok(status)
}
