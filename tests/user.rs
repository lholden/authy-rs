extern crate authy;

#[cfg(test)]
mod user {
    const API_URL: &str = "https://sandbox-api.authy.com";
    const API_KEY: &str = "bf12974d70818a08199d17d5e2bae630";

    use super::authy::{Client, Status, AuthyError};
    use super::authy::user;

    #[test]
    fn new() {
        let c = Client::new(API_URL, API_KEY);
        let (status, _) = user::new(&c, "user@domain.com", "317-338-9302", "54", false).expect("User to be created");

        assert!(status.success);
    }


    #[test]
    fn new_bad_user() {
        let c = Client::new(API_URL, API_KEY);
        let res = user::new(&c, "domain.com", "317-338-9302", "54", false);
        
        match res {
            Err(AuthyError::BadRequest(Status{success, message, ..})) => {
                assert!(!success);
                assert_eq!(message, "User was not valid");
            },
            _ => unreachable!("Expecting AuthyError::BadRequest"),
        };
    }

    #[test]
    fn delete() {
        let c = Client::new(API_URL, API_KEY);
        let (status, user) = user::new(&c, "user2341@domain.com", "317-338-2341", "54", false).expect("User to be created");
        assert!(status.success);

        let status = user::delete(&c, user.id).expect("User to be deleted");
        assert!(status.success);
    }

    #[test]
    fn status() {
        let c = Client::new(API_URL, API_KEY);
        let (status, user) = user::new(&c, "user@domain.com", "317-338-9302", "54", false).expect("User to be created");
        assert!(status.success);

        let (status, user_status) = user::status(&c, user.id).expect("User to have a status");
        assert!(status.success);

        assert_eq!(user_status.account_disabled, false);
    }

    #[test]
    fn verify() {
        let c = Client::new(API_URL, API_KEY);
        let (status, user) = user::new(&c, "user@domain.com", "317-338-9302", "54", false).expect("User to be created");
        assert!(status.success);

        let status = user::verify(&c, user.id, "0000000").expect("Valid token");
        assert!(status.success);

        let status = user::verify(&c, user.id, "12345");
        assert!(status.is_err());
    }
}
