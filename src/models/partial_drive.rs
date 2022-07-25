use serde::{Deserialize, Serialize};

use crate::models::RateLimiter;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartialDrive {
    pub drive_id: String,
    /// Host level path for the guest drive
    pub path_on_host: Option<String>,
    pub rate_limiter: Option<Box<RateLimiter>>,
}

impl PartialDrive {
    pub fn new(drive_id: String) -> Self {
        Self {
            drive_id,
            ..Default::default()
        }
    }
}
