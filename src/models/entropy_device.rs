use serde::{Deserialize, Serialize};

use crate::models;

/// Defines an entropy device.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EntropyDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limiter: Option<Box<models::RateLimiter>>,
}
