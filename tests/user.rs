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
}
