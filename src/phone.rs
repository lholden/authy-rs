//! The 'high-level' bindings to the Authy Phone Verification and Intelligence APIs.
//!
//! Please see [api::phone](../api/phone/index.html) for more details.

use error::AuthyError;
use client::Client;
use api;
pub use api::phone::{ContactType, PhoneStart};

/// Returned when finding a phone number.
///
/// Please see [api::phone](../api/phone/index.html) for more details.
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Phone {
    pub country_code: u16,
    pub phone_number: String,
    pub phone_type: String,
    pub provider: Option<String>,
    pub ported: bool,
}

impl Phone {
    /// Request information on a phone number.
    ///
    /// Please see [api::phone::info](../api/phone/fn.info.html) for more details.
    pub fn find(c: &Client, country_code: u16, phone: &str) -> Result<Phone, AuthyError> {
        let (status, info) = api::phone::info(c, country_code, phone, None)?;
        assert!(status.success);

        Ok(Phone {
            country_code: country_code,
            phone_number: phone.into(),
            phone_type: info.phone_type,
            provider: info.provider,
            ported: info.ported
        })
    }

    /// Initiate a phone verification check.
    ///
    /// Please see [api::phone::start](../api/phone/fn.start.html) for more details.
    pub fn start(&self, c: &Client, via: ContactType, code_length: Option<u8>, locale: Option<&str>) -> Result<PhoneStart, AuthyError> {
        let (status, phone_start) = api::phone::start(c, via, self.country_code, &self.phone_number, code_length, locale)?;
        assert!(status.success);

        Ok(phone_start)
    }

    /// Verify phone verification code sent to user.
    ///
    /// Please see [api::phone::check](../api/phone/fn.check.html) for more details.
    pub fn check(&self, c: &Client, code: &str) -> Result<(), AuthyError> {
        let status = api::phone::check(c, self.country_code, &self.phone_number, code)?;
        assert!(status.success);

        Ok(())
    }
}
