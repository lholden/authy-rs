use serde_json;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "protected";

#[derive(Debug, PartialEq, Deserialize)]
pub struct Details {
    pub app_id: u32,
    pub name: String,
    pub onetouch_enabled: bool,
    pub plan: String,
    pub sms_enabled: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
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
    let (status, res) = client.get(PREFIX, "app/details")?;

    let details = serde_json::from_value(res["app"].clone())?;

    Ok((status, details))
}

pub fn stats(client: &Client) -> Result<(Status, Vec<Stats>), AuthyError> {
    let (status, res) = client.get(PREFIX, "app/stats")?;

    let stats = serde_json::from_value(res["stats"].clone())?;

    Ok((status, stats))
}
