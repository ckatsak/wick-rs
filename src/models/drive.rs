use serde::{Deserialize, Serialize};

use crate::models::RateLimiter;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Drive {
    pub drive_id: String,
    /// Represents the caching strategy for the block device.
    pub cache_type: Option<CacheType>,
    pub is_read_only: bool,
    pub is_root_device: bool,
    /// Represents the unique id of the boot partition of this device. It is optional and it will
    /// be taken into account only if the `is_root_device` field is true.
    pub partuuid: Option<String>,
    /// Host level path for the guest drive
    pub path_on_host: String,
    pub rate_limiter: Option<Box<RateLimiter>>,
    /// Type of the IO engine used by the device. `Async` is supported on host kernels newer than
    /// 5.10.51.
    pub io_engine: Option<IoEngine>,
}

impl Drive {
    pub fn new(
        drive_id: String,
        is_read_only: bool,
        is_root_device: bool,
        path_on_host: String,
    ) -> Self {
        Self {
            drive_id,
            is_read_only,
            is_root_device,
            path_on_host,
            ..Default::default()
        }
    }
}

/// Represents the caching strategy for the block device.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum CacheType {
    #[default]
    #[serde(rename = "Unsafe")]
    Unsafe,
    #[serde(rename = "Writeback")]
    Writeback,
}

/// Type of the IO engine used by the device. `Async` is supported on host kernels newer than
/// 5.10.51.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum IoEngine {
    #[default]
    #[serde(rename = "Sync")]
    Sync,
    #[serde(rename = "Async")]
    Async,
}
