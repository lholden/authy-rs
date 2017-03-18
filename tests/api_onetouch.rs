extern crate authy;

#[cfg(test)]
mod onetouch {
    const API_URL: &'static str = "https://sandbox-api.authy.com";
    const API_KEY: &'static str = "bf12974d70818a08199d17d5e2bae630";

    use super::authy::Client;
    use super::authy::api::user;
    use super::authy::api::onetouch;

    #[test]
    #[ignore]
    // sandbox key has onetouch forbidden
    fn request() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 1000;
        let (status, user) = user::new(&c, "user@domain.com", 54, "317-338-9302", false).expect("User to be created");
        assert!(status.success);

        let (status, request) = onetouch::request(&c, user.id, "Hello!", None, None, None, None).expect("Request");
        assert!(status.success);

        println!("{:#?}", request);
    }
}
