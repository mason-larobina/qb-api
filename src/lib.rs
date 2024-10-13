#![doc = include_str!("../README.md")]

mod api;
pub mod data;
mod error;
pub mod queries;
pub mod traits;

#[cfg(test)]
pub(crate) mod tests;

pub use api::Api;
pub use error::Error;
