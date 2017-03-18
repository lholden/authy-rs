//! # Authy
//!
//! Bindings for the Authy two factor web service
//!
//! ## Usage
//!
//! You will need your API key for your application on [authy.com](https://authy.com).
//!
//! Be sure to add the authy crate to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! authy = "*"
//! ```
//!
//! Usage example:
//!
//! ```rust
//! extern crate authy;
//! use authy::Client;
//! use authy::user::User;
//!
//! fn main() {
//!     let api_url = "https://sandbox-api.authy.com";
//!     let api_key = "bf12974d70818a08199d17d5e2bae630";
//!
//!     let c = Client::new(api_url, api_key);
//!
//!     let country_code = 1;
//!     let email = "user@domain.com";
//!     let phone = "949-555-1234";
//!
//!     let mut user = User::create(&c, email, country_code, phone, true).unwrap();
//!     
//!     println!("We have a user: {:#?}", user);
//!
//!     let code = "000000"; // Pretend user has provided a valid code
//!     if user.verify(&c, code).unwrap() {
//!         println!("Congrats on being validated!");
//!     }
//!
//!     // Lets send out a sms token just for fun
//!     user.sms(&c, true, Some("login"), Some("Authy documentation example login")).unwrap();
//!
//! }
//! ```
#![feature(custom_attribute)]

extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod error;
pub use error::AuthyError;

mod client;
pub use client::{Client, Status};

pub mod api;

pub mod user;
pub mod phone;
