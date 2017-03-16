extern crate authy;

#[cfg(test)]
mod test {
    const API_URL: &str = "https://sandbox-api.authy.com";
    const API_KEY: &str = "bf12974d70818a08199d17d5e2bae630";

    use super::authy::{Client, user};

    #[test]
    fn new() {
        let c = Client::new(API_URL, API_KEY);
        let (status, user) = user::new(&c, "user@domain.com", "317-338-9302", "54", false).expect("User to be created");

        assert!(status.success);
        assert_eq!(user.id, 209);
    }

    #[test]
    fn delete() {
        let c = Client::new(API_URL, API_KEY);
        let (status, user) = user::new(&c, "user@domain.com", "317-338-1234", "54", false).expect("User to be created");
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
}
