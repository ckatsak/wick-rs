//! An **unofficial** client crate for
//! [Firecracker](https://github.com/firecracker-microvm/firecracker) v1.12.0, somewhat based on
//! the OpenAPI-generated client.

pub mod api;
pub mod models;

pub use api::client::Client;
pub use api::error::ApiError;
pub use api::error::Error;
pub use api::Api;
