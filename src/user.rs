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

    let (status, res) = client.post(PREFIX, "users/new", Some(params))?;

    let user = serde_json::from_value(res["user"].clone())?;

    Ok((status, user))
}

pub fn delete(client: &Client, user_id: u32) -> Result<Status, AuthyError> {
    let (status, _) = client.post(PREFIX, &format!("users/{}/delete", user_id), None)?;

    Ok(status)
}

pub fn status(client: &Client, user_id: u32) -> Result<(Status, UserStatus), AuthyError> {
    let (status, res) = client.get(PREFIX, &format!("users/{}/status", user_id))?;

    let user_status = serde_json::from_value(res["status"].clone())?;

    Ok((status, user_status))
}

pub fn verify(client: &Client, user_id: u32, token: &str) -> Result<Status, AuthyError> {
    let (status, _) = client.get(PREFIX, &format!("verify/{token}/{user_id}", token = token, user_id = user_id))?;

    Ok(status)
}
