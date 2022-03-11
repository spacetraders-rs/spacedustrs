//! This crate is an api wrapper around the [SpaceTraders] API game, version 2.
//!
//! The goal is to wrap all available endpoints in strongly typed requests and responses that
//! we expect from rust.
//!
//! [SpaceTraders]: https://spacetraders.io/
#![warn(missing_docs)]

pub mod client;
pub mod errors;
pub mod requests;
pub mod responses;
pub mod shared;
