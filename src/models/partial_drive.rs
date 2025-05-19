use camino::Utf8PathBuf;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PartialDrive {
    pub drive_id: CompactString,
    /// Host level path for the guest drive. This field is optional for virtio-block config and
    /// should be omitted for vhost-user-block configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_on_host: Option<Utf8PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limiter: Option<Box<models::RateLimiter>>,
}

impl PartialDrive {
    #[inline]
    pub fn new(drive_id: impl Into<CompactString>) -> Self {
        Self {
            drive_id: drive_id.into(),
            path_on_host: None,
            rate_limiter: None,
        }
    }
}
