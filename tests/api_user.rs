extern crate authy;

#[cfg(test)]
mod user {
    const API_URL: &'static str = "https://sandbox-api.authy.com";
    const API_KEY: &'static str = "bf12974d70818a08199d17d5e2bae630";

    use std::collections::HashMap;

    use super::authy::{Client, Status, AuthyError};
    use super::authy::api::user::{self, ActivityType};

    #[test]
    fn new() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, _) = user::create(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");

        assert!(status.success);
    }

    #[test]
    fn new_bad_user() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let res = user::create(&c, "domain.com", 54, "317-338-9302", false);
        
        match res {
            Err(AuthyError::BadRequest(Status{success, message, ..})) => {
                assert!(! success);
                assert_eq!(message, "User was not valid");
            },
            o => unreachable!("Expecting AuthyError::BadRequest, got: {:?}", o),
        };
    }

    #[test]
    fn delete() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, user) = user::create(&c, "user2341@domain.com", 54, "317-338-2341", false).expect("User to be created");
        assert!(status.success);

        let status = user::delete(&c, user.id).expect("User to be deleted");
        assert!(status.success);
    }

    #[test]
    fn delete_bad_user() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let res = user::delete(&c, 0);

        match res {
            Err(AuthyError::UserNotFound(Status{success, message, ..})) => {
                assert!(! success);
                assert_eq!(message, "User not found.");
            },
            o => unreachable!("Expecting AuthyError::UserNotFound, got: {:?}", o),
        };
    }

    #[test]
    fn status() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, user) = user::create(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");
        assert!(status.success);

        let (status, user_status) = user::status(&c, user.id).expect("User to have a status");
        assert!(status.success);

        assert_eq!(user_status.account_disabled, false);
    }

    #[test]
    fn verify() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, user) = user::create(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");
        assert!(status.success);

        let status = user::verify(&c, user.id, "0000000").expect("Valid token");
        assert!(status.success);
    }

    #[test]
    fn verify_invalid_token() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, user) = user::create(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");
        assert!(status.success);

        let res = user::verify(&c, user.id, "123456");

        match res {
            Err(AuthyError::UnauthorizedKey(Status{success, message, ..})) => {
                assert!(! success);
                assert_eq!(message, "Token is invalid");
            },
            o => unreachable!("Expecting AuthyError::UnauthorizedKey, got: {:?}", o),
        };
    }

    #[test]
    fn sms() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, user) = user::create(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");
        assert!(status.success);

        let (status, sms) = user::sms(&c, user.id, false, None, None).expect("Phone");
        assert!(status.success);

        assert_eq!(sms.cellphone, "+54-XXX-XXX-XX02");
    }

    #[test]
    fn sms_action() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, user) = user::create(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");
        assert!(status.success);

        let (status, sms) = user::sms(&c, user.id, true, Some("an_action"), Some("a_message")).expect("Phone");
        assert!(status.success);

        assert_eq!(sms.cellphone, "+54-XXX-XXX-XX02");
    }

    #[test]
    fn call() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, user) = user::create(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");
        assert!(status.success);

        let (status, sms) = user::call(&c, user.id, false, None, None).expect("Phone");
        assert!(status.success);

        assert_eq!(sms.cellphone, "+54-XXX-XXX-XX02");
    }

    #[test]
    fn call_action() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, user) = user::create(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");
        assert!(status.success);

        let (status, sms) = user::call(&c, user.id, true, Some("an_action"), Some("a_message")).expect("Phone");
        assert!(status.success);

        assert_eq!(sms.cellphone, "+54-XXX-XXX-XX02");
    }

    #[test]
    #[ignore]
    // This works with my real API key, just not on the sandbox
    fn register_activity() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, user) = user::create(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");
        assert!(status.success);


        let mut data: HashMap<&str, String> = HashMap::new();
        data.insert("my_user_id", "1234".into());

        let status = user::register_activity(&c, user.id, Some(&data), ActivityType::PasswordReset, "192.168.0.1").expect("Status");
        assert!(status.success);
    }
}
