use serde_json;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "protected";

#[derive(Debug, Deserialize)]
pub struct Details {
    pub app_id: u32,
    pub name: String,
    pub onetouch_enabled: bool,
    pub plan: String,
    pub sms_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub month: String,
    pub year: u16,

    pub api_calls_count: u32,
    pub auths_count: u32,
    pub calls_count: u32,
    pub sms_count: u32,
    pub users_count: u32,
}

pub fn details(client: &Client) -> Result<(Status, Details), AuthyError> {
    let body = client.get(PREFIX, "app/details")?;

    let status = serde_json::from_value(body.clone())?;
    let details = serde_json::from_value(body["app"].clone())?;

    Ok((status, details))
}

pub fn stats(client: &Client) -> Result<(Status, Vec<Stats>), AuthyError> {
    let body = client.get(PREFIX, "app/stats")?;

    let status = serde_json::from_value(body.clone())?;
    let stats = serde_json::from_value(body["stats"].clone())?;

    Ok((status, stats))
}
