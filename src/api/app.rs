//! Bindings to app portions of the [Authy TOTP service api](https://www.twilio.com/docs/api/authy/authy-totp).
//!
//! Much of the documentation for this module comes from the Authy TOTP service
//! documentation.
use serde_json;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "protected";

/// returned when requesting the application details
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Details {
    pub app_id: u32,
    pub name: String,
    pub onetouch_enabled: bool,
    pub plan: String,
    pub sms_enabled: bool,
}

/// returned when requesting the application stats.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub month: String,
    pub year: u16,

    pub api_calls_count: u32,
    pub auths_count: u32,
    pub calls_count: u32,
    pub sms_count: u32,
    pub users_count: u32,
}

/// Get the details for an Authy application.
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-totp#application-details
pub fn details(client: &Client) -> Result<(Status, Details), AuthyError> {
    let (status, res) = client.get(PREFIX, "app/details", None)?;

    let details = serde_json::from_value(res["app"].clone())?;

    Ok((status, details))
}

/// Get stats for an Authy application.
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-totp#application-stats
pub fn stats(client: &Client) -> Result<(Status, Vec<Stats>), AuthyError> {
    let (status, res) = client.get(PREFIX, "app/stats", None)?;

    let stats = serde_json::from_value(res["stats"].clone())?;

    Ok((status, stats))
}
