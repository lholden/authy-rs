use error::AuthyError;
use client::Client;
use api;
pub use api::phone::{ContactType, PhoneStart};

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Phone {
    pub country_code: u16,
    pub phone_number: String,
    pub phone_type: String,
    pub provider: String,
    pub ported: bool,
}

impl Phone {
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

    pub fn start(&self, c: &Client, via: ContactType, code_length: Option<u8>, locale: Option<&str>) -> Result<PhoneStart, AuthyError> {
        let (status, phone_start) = api::phone::start(c, via, self.country_code, &self.phone_number, code_length, locale)?;
        assert!(status.success);

        Ok(phone_start)
    }

    pub fn check(&self, c: &Client, code: &str) -> Result<(), AuthyError> {
        let status = api::phone::check(c, self.country_code, &self.phone_number, code)?;
        assert!(status.success);

        Ok(())
    }
}
