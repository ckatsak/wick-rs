//! An **unofficial** client crate for
//! [Firecracker](https://github.com/firecracker-microvm/firecracker) v1.13.0.

pub mod api;
pub mod models;

pub use api::client::Client;
pub use api::error::ApiError;
pub use api::error::Error;
pub use api::Api;
