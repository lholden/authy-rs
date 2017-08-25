//! Bindings to the [Phone Verification](https://www.twilio.com/docs/api/authy/authy-phone-verification-api) and [Phone Intelligence](https://www.twilio.com/docs/api/authy/authy-phone-intelligence-api) apis.
//!
//! Much of the documentation for this module comes from the Authy TOTP service
//! documentation.
use std::fmt::{self, Display};

use serde_json;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "protected";

/// Returned when requesting info on a phone number
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PhoneInfo {
    /// Phone number type. It can be voip, landline, cellphone, unknown.
    #[serde(rename = "type")]
    pub phone_type: String,

    /// Name of the service provider.
    pub provider: Option<String>,

    /// Whether the phone number was ported or not.
    pub ported: bool,
}

/// Returned when initiating verification of a phone number.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PhoneStart {
    pub is_ported: bool,
    pub is_cellphone: bool,
}

/// The contact type used when verifying a phone number
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

/// Request information on a phone number.
///
/// The Authy Phone Intelligence API provides information about a phone number.
/// We return 3 key pieces of information.
///
/// * type of phone number [cell phone | landline | voip]
/// * provider of the number, e.g. "AT&T Wireless"
/// * If the number has been ported from a previous provider.
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-phone-intelligence-api
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

/// Initiate a phone verification check.
///
/// The Authy Phone Verification API allows you to verify that the user has the
/// device in their possession. The Authy Phone Verification API lets you
/// request a verification code to be sent to the user and also verify that the
/// code received by the user is valid.
///
/// When you want to verify a user's phone you start by requesting a
/// verification code for that user's phone number. The verification code is
/// valid for 10 minutes. Subsequent calls to the API within the expiration
/// time will send the same verification code.
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-phone-verification-api#requesting-and-verifying-the-verification-code
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

/// Verify phone verification code sent to user.
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-phone-verification-api#verifying-code-sent-to-the-user
pub fn check(client: &Client, country_code: u16, phone: &str, code: &str) -> Result<Status, AuthyError> {
    let mut params: Vec<(String, String)> = vec![];
    params.push(("country_code".into(), country_code.to_string()));
    params.push(("phone_number".into(), phone.into()));
    params.push(("verification_code".into(), code.into()));

    let (status, _) = client.get(PREFIX, "phones/verification/check", Some(params))?;

    Ok(status)
}
