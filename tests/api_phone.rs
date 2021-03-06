extern crate authy;

#[cfg(test)]
mod phone {
    const API_URL: &'static str = "https://sandbox-api.authy.com";
    const API_KEY: &'static str = "bf12974d70818a08199d17d5e2bae630";

    use super::authy::{Client, Status, AuthyError};
    use super::authy::api::phone::{self, ContactType};

    #[test]
    fn info() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, info) = phone::info(&c, 54, "317-338-9302", None).expect("PhoneInfo");
        assert!(status.success);
        assert_eq!(info.phone_type, "landline");

        // On the sandbox, info doesn't validate the country or phone are
        // correct, just that the params exist. There is no way to write a
        // fail case.
    }

    #[test]
    fn start_sms() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, info) = phone::start(&c, ContactType::SMS, 54, "317-338-9302", None, None).expect("PhoneVerification");
        assert!(status.success);
        assert_eq!(info.carrier, "Google Voice");
        assert_eq!(info.message, "Text message sent to +54 317-338-9302.");
    }


    #[test]
    fn start_fail() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let res = phone::start(&c, ContactType::Call, 54, "555-9302", None, None);

        match res {
            Err(AuthyError::BadRequest(Status{success, message, ..})) => {
                assert!(! success);
                assert_eq!(message, "Phone number is invalid");
            }
            o => unreachable!("Expecting AuthyError::BadRequest, got: {:?}", o),
        }
    }

    #[test]
    #[ignore]
    // Gave up - This fails on sandbox because the verification start
    // doesn't actually kick off a verification.
    fn check() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, _) = phone::start(&c, ContactType::Call, 54, "317-555-9302", None, None).expect("PhoneVerification");
        assert!(status.success);

        let status = phone::check(&c, 54, "317-555-9302", "0000").expect("Status");
        assert!(status.success);
    }
}
