extern crate authy;

#[cfg(test)]
mod phone {
    const API_URL: &'static str = "https://sandbox-api.authy.com";
    const API_KEY: &'static str = "bf12974d70818a08199d17d5e2bae630";

    use super::authy::{Client, Status, AuthyError};
    use super::authy::api::phone::{self, ContactType};

    #[test]
    fn info() {
        let c = Client::new(API_URL, API_KEY);
        let (status, info) = phone::info(&c, 54, "317-338-9302", None).expect("PhoneInfo");
        assert!(status.success);
        assert_eq!(info.phone_type, "landline");

        // On the sandbox, info doesn't validate the country or phone are
        // correct, just that the params exist. There is no way to write a
        // fail case.
    }

    #[test]
    fn start_sms() {
        let c = Client::new(API_URL, API_KEY);
        let (status, info) = phone::start(&c, ContactType::SMS, 54, "317-338-9302", None, None).expect("PhoneVerification");
        assert!(status.success);
        assert!(! info.is_ported);
    }


    #[test]
    fn start_fail() {
        let c = Client::new(API_URL, API_KEY);
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
        let c = Client::new(API_URL, API_KEY);
        let (status, _) = phone::start(&c, ContactType::Call, 54, "317-555-9302", None, None).expect("PhoneVerification");
        assert!(status.success);

        let status = phone::check(&c, 54, "317-555-9302", "0000").expect("Status");
        assert!(status.success);
    }
}
