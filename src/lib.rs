//! An **unofficial** client crate for [Firecracker](https://github.com/firecracker-microvm/firecracker)
//! v1.1.0, somewhat based on the OpenAPI-generated client.

pub mod api;
pub mod models;

pub use api::client::Error;
pub use api::client::MicrovmClient;
