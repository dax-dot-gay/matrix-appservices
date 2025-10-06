#![warn(missing_docs)]

//! Matrix Appservice SDK
//!
//! A convenience wrapper around Matrix Appservice functionality

#[allow(missing_docs)]
pub use matrix_sdk;

///
pub mod types;

///
mod errors;
pub(crate) use errors::Result;
pub use errors::Error;
