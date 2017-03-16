extern crate authy;

#[cfg(test)]
mod test {
    const API_URL: &str = "https://sandbox-api.authy.com";
    const API_KEY: &str = "bf12974d70818a08199d17d5e2bae630";

    use super::authy::{Client, app};

    #[test]
    fn details() {
        let c = Client::new(API_URL, API_KEY);
        let (status, details) = app::details(&c).expect("Details of authy app");
        assert!(status.success);
        assert_eq!(details.name, "Sandbox App 1");
    }

    #[test]
    fn stats() {
        let c = Client::new(API_URL, API_KEY);
        let (status, stats) = app::stats(&c).expect("Stats of authy app");
        assert!(status.success);
        stats.iter().find(|s| s.month == "February" && s.year == 2017).expect("Find Feb of 2017");
    }
}
