//! Bindings to the [Authy TOTP service api](https://www.twilio.com/docs/api/authy/authy-totp).
//!
//! Much of the documentation for this module comes from the Authy TOTP service
//! documentation.
use std::fmt::{self, Display};

use serde_json;
use std::collections::HashMap;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "protected";

/// Returned when creating a new authy user. 
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserNew {
/// This id is unique per API KEY and should be stored in your database.
    pub id: u32,
}

/// Returned when requesting the status of an authy user.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserStatus {
    /// The authy id for the user.
    #[serde(rename = "authy_id")]
    pub id: u32,

    /// true when the user has used a valid code before.
    pub confirmed: bool,

    /// true when the Authy Mobile/Desktop App was registered.
    pub registered: bool,
    
    /// Has the account been marked for deletion
    pub account_disabled: bool,

    /// The country code listed for the user.
    pub country_code: u16,
    
    /// The last 4 of the phone number registered to the account.
    pub phone_number: String,

    /// (Unknown, API documentation doesn't list)
    pub has_hard_token: bool,

    /// List of devices, options are: android, android_tablet, ios, authy_chrome, sms.
    pub devices: Vec<String>,
}

/// Returned when sending a verification code to a user via SMS or Call.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PhoneCall {
    /// The phone number used to send the message.
    pub cellphone: String,

    /// The name of the most recent device used by the user. This is only 
    /// returned when the SMS was ignored.
    pub device: Option<String>,

    /// True if the request was ignored.
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

#[deprecated]
/// Please use `create`.
pub fn new(client: &Client, email: &str, country_code: u16, phone: &str, send_instructions: bool) -> Result<(Status, UserNew), AuthyError> {
    create(client, email, country_code, phone, send_instructions)
}

/// Creates a new Authy user.
///
/// Before you can secure a user's login you need to create an Authy user. 
/// Authy requires you send an email, phone number and country code for each 
/// Authy user. In response you get an Authy ID which you must then store with
/// your user's profile in your own application.
///
/// Note: You need to store the unchanging authy_id against the user profile 
/// in your database or directory: You will use this ID every time you are 
/// verifying the user's token. For privacy reasons, your users can change 
/// their phone number registered with Authy without your knowledge by using 
/// the Authy mobile or desktop app or the Authy.com phone change security 
/// review. Their Authy ID may be used for other services as well.
///
/// A user can have multiple e-mails but only one cellphone. Two separate api
/// calls to register a user with the same cellphone and different e-mails will
/// return the same authy_id and store both emails for that Authy user.
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-totp#enabling-two-factor
///
/// Example:
/// 
/// ```rust,ignore
/// let mut c = Client::new(API_URL, API_KEY);
/// let (status, user) = user::new(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");
///
/// println!("My new authy user is: {}", user.id);
/// ```
pub fn create(client: &Client, email: &str, country_code: u16, phone: &str, send_instructions: bool) -> Result<(Status, UserNew), AuthyError> {
    let mut params: Vec<(String, String)> = vec![];
    params.push(("user[email]".into(), email.into()));
    params.push(("user[cellphone]".into(), phone.into()));
    params.push(("user[country_code]".into(), country_code.to_string()));
    if send_instructions {
        params.push(("send_install_link_via_sms".into(), "true".into()));
    }

    let (status, res) = client.post(PREFIX, "users/new", None, Some(params))?;

    let user = serde_json::from_value(res["user"].clone())?;

    Ok((status, user))
}

/// Deletes an Authy user.
///
/// If you want to remove users from your application you can use the delete
/// API. Note, deleting a user will NOT immediately disable token 
/// verifications, as a 24 hour delay is typical before the user is permanently
/// removed from the application. If you need to immediately disable a user and
/// have not built this functionality into your user management system, the 
/// Authy dashboard "Delete user" function can be used, and after approving the
/// email confirmation, 2FA for the user will be immediately disabled. 
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-totp#deleting-user
pub fn delete(client: &Client, id: u32) -> Result<Status, AuthyError> {
    let (status, _) = client.post(PREFIX, &format!("users/{}/delete", id), None, None)?;

    Ok(status)
}

/// Status of an Authy user.
///
/// This will return various details of an Authy user such as their 
/// registration status, their country code, and the last 4 of their phone 
/// number.
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-totp#user-status
pub fn status(client: &Client, id: u32) -> Result<(Status, UserStatus), AuthyError> {
    let (status, res) = client.get(PREFIX, &format!("users/{}/status", id), None)?;

    let user_status = serde_json::from_value(res["status"].clone())?;

    Ok((status, user_status))
}

/// Verify an authentication token.
///
/// To verify a token simply pass in the token that the user entered and the 
/// authy id of the user (which should have stored in your database when you 
/// registered the user above). Authy will use HTTP status codes for the 
/// response.
///
/// To prevent user from being locked out, until the user successfully logs 
/// in once using Authy this call will return 200 (valid token). If you wish
/// to verify token regardless, see below to see how to force verification. 
/// HTTP 200 means valid token and HTTP 401 means invalid token
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-totp#verifying-a-token
pub fn verify(client: &Client, id: u32, token: &str) -> Result<Status, AuthyError> {
    let (status, _) = client.get(PREFIX, &format!("verify/{token}/{id}", token = token, id = id), None)?;

    Ok(status)
}

fn phone(client: &Client, kind: &str, id: u32, force: bool, action: Option<&str>, action_message: Option<&str>) -> Result<(Status, PhoneCall), AuthyError> {
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

/// Send token to user via SMS.
///
/// Once an Authy ID has been generated for a user, you can provide a 
/// two-factor stage to a login. If the user downloads and installs the Authy
/// smartphone application, it will generate the codes required. However, for
/// users that don't own a smartphone, Authy allows you to use text messages
/// to send the one time passcode. By default this call will be ignored if the
/// user has downloaded and registered the Authy smartphone application
/// against their phone number. However you can override this behavior.
///
/// Custom Actions
///
/// You can pass `action` and `action_message` (optional) to send a code that
/// is only valid for the given action. This is useful if you require codes to
/// perform different actions on your app. When using this option you have to
/// pass the same action when verifying the code.
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-totp#requesting-sms-codes
pub fn sms(client: &Client, id: u32, force: bool, action: Option<&str>, action_message: Option<&str>) -> Result<(Status, PhoneCall), AuthyError> {
    phone(client, "sms", id, force, action, action_message)
}

/// Send token to user via phone call.
///
/// For users that don't own a smartphone, and are having trouble with SMS 
/// Tokens, Authy allows you to use phone calls instead. This call will be 
/// ignored if the user is using the Authy Mobile app.
///
/// Custom Actions
///
/// You can pass `action` and `action_message` (optional) to send a code that
/// is only valid for the given action. This is useful if you require codes to
/// perform different actions on your app. When using this option you have to
/// pass the same action when verifying the code.
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-totp#phone-call-tokens
pub fn call(client: &Client, id: u32, force: bool, action: Option<&str>, action_message: Option<&str>) -> Result<(Status, PhoneCall), AuthyError> {
    phone(client, "call", id, force, action, action_message)
}

/// Register user activity with authy.
///
/// Optionally you can register some of the activities that your user do on
/// your application. This helps us to identify fraudulent behaviours. For
/// example if you register that a user reset his password and then he tries to
/// change his phone with Authy we can know that something weird is happening.
///
/// Please see the Authy documentation for more details:
/// https://www.twilio.com/docs/api/authy/authy-totp#register-user-activities
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
