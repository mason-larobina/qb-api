#![doc = include_str!("../README.md")]

mod api;
pub mod data;
mod error;
pub mod queries;
pub mod traits;

pub use api::Api;
pub use error::Error;
