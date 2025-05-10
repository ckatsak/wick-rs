use crate::models;
use serde::{Deserialize, Serialize};

/// EntropyDevice : Defines an entropy device.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntropyDevice {
    #[serde(rename = "rate_limiter", skip_serializing_if = "Option::is_none")]
    pub rate_limiter: Option<Box<models::RateLimiter>>,
}

impl EntropyDevice {
    /// Defines an entropy device.
    pub fn new() -> Self {
        Default::default()
    }
}
