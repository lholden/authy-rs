# Authy
[![](https://docs.rs/authy/badge.svg)](https://docs.rs/authy) [![](https://img.shields.io/crates/v/authy.svg)](https://crates.io/crates/authy) [![](https://img.shields.io/crates/l/authy.svg)](https://raw.githubusercontent.com/lholden/authy-rs/master/LICENSE) [![](https://travis-ci.org/lholden/authy-rs.svg?branch=master)](https://travis-ci.org/lholden/authy-rs)

Bindings for the Authy two factor web service

## Usage

Please see the [Documentation](https://docs.rs/authy) for more details.

You will need your API key for your application on [authy.com](https://authy.com).

Be sure to add the authy crate to your `Cargo.toml`:

```toml
[dependencies]
authy = "*"
```

Usage example:

```rust
extern crate authy;
use authy::Client;
use authy::user::User;

fn main() {
    let api_url = "https://sandbox-api.authy.com";
    let api_key = "bf12974d70818a08199d17d5e2bae630";

    let c = Client::new(api_url, api_key);

    let country_code = 1;
    let email = "user@domain.com";
    let phone = "949-555-1234";

    let mut user = User::create(&c, email, country_code, phone, true).unwrap();

    println!("We have a user: {:#?}", user);

    let code = "000000"; // Pretend user has provided a valid code
    if user.verify(&c, code).unwrap() {
        println!("Congrats on being validated!");
    }

    // Lets send out a sms token just for fun
    // Must be using a real API key on the production authy server for this to
    // actually send out anything.
    user.sms(&c, true, Some("login"), Some("Authy documentation example login")).unwrap();
}
```
