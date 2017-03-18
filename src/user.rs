use std::collections::HashMap;

use error::AuthyError;
use client::{Client, Status};
use api;
pub use api::user::{PhoneCall, ActivityType};

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u32,

    pub confirmed: bool,
    pub registered: bool,
    pub account_disabled: bool,

    pub country_code: u16,
    pub phone_number: String,

    pub has_hard_token: bool,

    pub devices: Vec<String>,
}

impl User {
    pub fn create(c: &Client, email: &str, country_code: u16, phone: &str, send_instructions: bool) -> Result<User, AuthyError> {
        let (status, user_new) = api::user::new(c, email, country_code, phone, send_instructions)?;
        assert!(status.success);

        Self::find(c, user_new.id)
    }

    pub fn find(c: &Client, id: u32) -> Result<User, AuthyError> {
        let mut u = User { id: id, ..User::default() };
        u.update(c)?;
        Ok(u)
    }

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

    pub fn delete(&mut self, c: &Client) -> Result<(), AuthyError> {
        let status = api::user::delete(c, self.id)?;
        assert!(status.success);
        self.update(c)?;
        Ok(())
    }

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

    pub fn sms(&self, c: &Client, force: bool, action: Option<&str>, action_message: Option<&str>) -> Result<PhoneCall, AuthyError> {
        let (status, phone) = api::user::sms(c, self.id, force, action, action_message)?;
        assert!(status.success);

        Ok(phone)
    }

    pub fn call(&self, c: &Client, force: bool, action: Option<&str>, action_message: Option<&str>) -> Result<PhoneCall, AuthyError> {
        let (status, phone) = api::user::call(c, self.id, force, action, action_message)?;
        assert!(status.success);

        Ok(phone)
    }

    pub fn register_activity(&self, c: &Client, data: Option<&HashMap<&str, String>>, activity_type: ActivityType, user_ip: &str) -> Result<(), AuthyError> {
        let status = api::user::register_activity(c, self.id, data, activity_type, user_ip)?;
        assert!(status.success);
        Ok(())
    }
}
