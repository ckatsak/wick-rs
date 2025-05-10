use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialDrive {
    #[serde(rename = "drive_id")]
    pub drive_id: String,
    /// Host level path for the guest drive. This field is optional for virtio-block config and
    /// should be omitted for vhost-user-block configuration.
    #[serde(rename = "path_on_host", skip_serializing_if = "Option::is_none")]
    pub path_on_host: Option<String>,
    #[serde(rename = "rate_limiter", skip_serializing_if = "Option::is_none")]
    pub rate_limiter: Option<Box<models::RateLimiter>>,
}

impl PartialDrive {
    pub fn new(drive_id: String) -> Self {
        Self {
            drive_id,
            path_on_host: None,
            rate_limiter: None,
        }
    }
}
