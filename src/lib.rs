#![feature(custom_attribute)]

extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod error;
pub use error::AuthyError;

mod client;
pub use client::{Client, Status};

pub mod app;
pub mod user;
pub mod phone;
