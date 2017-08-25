//! The 'high-level' bindings to the Authy TOTP API.
//!
//! Please see [api::user](../api/user/index.html) for more details.

use std::collections::HashMap;

use error::AuthyError;
use client::{Client, Status};
use api;
pub use api::user::{PhoneCall, ActivityType};

/// An Authy user as part of the 'high level' Authy interface.
///
/// Please see [api::user](../api/user/index.html) for more details.
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// The authy id for the user.
    pub id: u32,

    /// true when the Authy Mobile/Desktop App was registered.
    pub registered: bool,

    /// true when the user has used a valid code before.
    pub confirmed: bool,

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

impl User {
    /// Create a new user with the Authy service. Returns a `User` populated
    /// with data from the 'status' API call. The id returned must be stored
    /// and used future interactions with Authy for the user.
    ///
    /// Please see [api::user::create](../api/user/fn.create.html)
    pub fn create(c: &Client, email: &str, country_code: u16, phone: &str, send_instructions: bool) -> Result<User, AuthyError> {
        let (status, user_new) = api::user::create(c, email, country_code, phone, send_instructions)?;
        assert!(status.success);

        Self::find(c, user_new.id)
    }

    /// Returns a `User` populated with data from the `status` API call for the
    /// given authy id.
    ///
    /// Please see [api::user::status](../api/user/fn.status.html)
    pub fn find(c: &Client, id: u32) -> Result<User, AuthyError> {
        let mut u = User { id: id, ..User::default() };
        u.update(c)?;
        Ok(u)
    }

    /// Updates the `User` with the latest data from authy's service.
    ///
    /// Please see [api::user::status](../api/user/fn.status.html)
    pub fn update(&mut self, c: &Client) -> Result<(), AuthyError> {
        let (status, u) = api::user::status(c, self.id)?;
        assert!(status.success);
         
        self.id = u.id;
        self.confirmed = u.confirmed;
        self.registered = u.registered;
        self.account_disabled = u.account_disabled;
        self.country_code = u.country_code;
        self.has_hard_token = u.has_hard_token;
        self.phone_number = u.phone_number.clone();
        self.devices = u.devices.clone();

        Ok(())
    } 

    /// Marks a user for deletion.
    ///
    /// Please see [api::user::delete](../api/user/fn.delete.html)
    pub fn delete(&mut self, c: &Client) -> Result<(), AuthyError> {
        let status = api::user::delete(c, self.id)?;
        assert!(status.success);
        self.update(c)?;
        Ok(())
    }

    /// Perform a verification request. Returns Ok(true) for a successful
    /// verification and Ok(false) when the verification code was invalid.
    ///
    /// Please see [api::user::verify](../api/user/fn.verify.html)
    pub fn verify(&mut self, c: &Client, token: &str) -> Result<bool, AuthyError> {
        match api::user::verify(c, self.id, token) {
            Ok(status) => {
                assert!(status.success);
                self.update(c)?;
                Ok(true)
            }
            Err(AuthyError::UnauthorizedKey(Status { ref message,.. }))
                if message == "Token is invalid" => Ok(false),
            Err(e) => Err(e)
        }
    }

    /// Requests that the Authy service send the user a verification code over
    /// SMS. This request will be ignored if the user is using the Authy
    /// Mobile app unless force is set to true.
    ///
    /// Please see [api::user::sms](../api/user/fn.sms.html)
    pub fn sms(&self, c: &Client, force: bool, action: Option<&str>, action_message: Option<&str>) -> Result<PhoneCall, AuthyError> {
        let (status, phone) = api::user::sms(c, self.id, force, action, action_message)?;
        assert!(status.success);

        Ok(phone)
    }

    /// Requests that the Authy service send the user a verification code over 
    /// the phone. This request will be ignored if the user is using the Authy 
    /// Mobile app unless force is set to true.
    ///
    /// Please see [api::user::call](../api/user/fn.call.html)
    pub fn call(&self, c: &Client, force: bool, action: Option<&str>, action_message: Option<&str>) -> Result<PhoneCall, AuthyError> {
        let (status, phone) = api::user::call(c, self.id, force, action, action_message)?;
        assert!(status.success);

        Ok(phone)
    }

    /// Optionally, you can register some of the activities that your user do
    /// on your application. This helps us to identify fraudulent behaviours.
    /// For example if you register that a user reset his password and then he
    /// tries to change his phone with Authy we can know that something weird
    /// is happening.
    ///
    /// Please see [api::user::register_activity](../api/user/fn.register_activity.html)
    pub fn register_activity(&self, c: &Client, data: Option<&HashMap<&str, String>>, activity_type: ActivityType, user_ip: &str) -> Result<(), AuthyError> {
        let status = api::user::register_activity(c, self.id, data, activity_type, user_ip)?;
        assert!(status.success);
        Ok(())
    }
}
