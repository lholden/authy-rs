use serde_json;
use std::collections::HashMap;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "protected/json";

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
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
