extern crate authy;

#[cfg(test)]
mod api_app {
    const API_URL: &str = "https://sandbox-api.authy.com";
    const API_KEY: &str = "bf12974d70818a08199d17d5e2bae630";

    use super::authy::{Client, Status, AuthyError};
    use super::authy::api::app;

    #[test]
    fn details() {
        let c = Client::new(API_URL, API_KEY);
        let (status, details) = app::details(&c).expect("Details of authy app");
        assert!(status.success);
        assert_eq!(details.name, "Sandbox App 1");
    }

    #[test]
    fn details_bad_key() {
        let c = Client::new(API_URL, "a_bad_key");
        let res = app::details(&c);

        match res {
            Err(AuthyError::UnauthorizedKey(Status{success, message, ..})) => {
                assert!(!success);
                assert_eq!(message, "Invalid API key");
            },
            other => {
                unreachable!("Expecting AuthyError::UnauthorizedKey: {:?}", other);
            },
        };
    }

    #[test]
    fn stats() {
        let c = Client::new(API_URL, API_KEY);
        let (status, stats) = app::stats(&c).expect("Stats of authy app");
        assert!(status.success);
        stats.iter().find(|s| s.month == "February" && s.year == 2017).expect("Find Feb of 2017");
    }
}
