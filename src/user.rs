use serde_json;
use std::collections::HashMap;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "protected";

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct UserStatus {
    pub authy_id: u32,
    pub confirmed: bool,
    pub registered: bool,
    pub account_disabled: bool,

    pub country_code: u16,
    pub phone_number: String,
    pub has_hard_token: bool,

    pub devices: Vec<String>,
}

pub fn new(client: &Client, email: &str, phone: &str, country: &str, send_install: bool) -> Result<(Status, User), AuthyError> {
    let mut params = HashMap::new();
    params.insert("user[email]", email);
    params.insert("user[cellphone]", phone);
    params.insert("user[country_code]", country);
    if send_install {
        params.insert("send_install_link_via_sms", "true");
    }

    let body = client.post(PREFIX, "users/new", Some(params))?;

    let status = serde_json::from_value(body.clone())?;
    let user = serde_json::from_value(body["user"].clone())?;

    Ok((status, user))
}

pub fn delete(client: &Client, user_id: u32) -> Result<Status, AuthyError> {
    let body = client.post(PREFIX, &format!("users/{}/delete", user_id), None)?;

    let status = serde_json::from_value(body.clone())?;
    Ok(status)
}

pub fn status(client: &Client, user_id: u32) -> Result<(Status, UserStatus), AuthyError> {
    let body = client.get(PREFIX, &format!("users/{}/status", user_id))?;

    let status = serde_json::from_value(body.clone())?;
    let user_status = serde_json::from_value(body["status"].clone())?;

    Ok((status, user_status))
}
