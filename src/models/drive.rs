use camino::Utf8PathBuf;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Drive {
    pub drive_id: CompactString,
    /// Represents the unique id of the boot partition of this device. It is optional and it will
    /// be taken into account only if the is_root_device field is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partuuid: Option<String>,
    pub is_root_device: bool,
    /// Represents the caching strategy for the block device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_type: Option<CacheType>,
    /// Is block read only. This field is required for virtio-block config and should be omitted
    /// for vhost-user-block configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    /// Host level path for the guest drive. This field is required for virtio-block config and
    /// should be omitted for vhost-user-block configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_on_host: Option<Utf8PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limiter: Option<Box<models::RateLimiter>>,
    /// Type of the IO engine used by the device. \"Async\" is supported on host kernels newer
    /// than 5.10.51. This field is optional for virtio-block config and should be omitted for
    /// vhost-user-block configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_engine: Option<IoEngine>,
    /// Path to the socket of vhost-user-block backend. This field is required for vhost-user-block
    /// config should be omitted for virtio-block configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<Utf8PathBuf>,
}

impl Drive {
    pub fn new(drive_id: impl Into<CompactString>, is_root_device: bool) -> Self {
        Self {
            drive_id: drive_id.into(),
            partuuid: None,
            is_root_device,
            cache_type: None,
            is_read_only: None,
            path_on_host: None,
            rate_limiter: None,
            io_engine: None,
            socket: None,
        }
    }
}

/// Represents the caching strategy for the block device.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum CacheType {
    #[default]
    Unsafe,
    Writeback,
}

/// Type of the IO engine used by the device. "Async" is supported on host kernels newer than
/// 5.10.51. This field is optional for virtio-block config and should be omitted for
/// vhost-user-block configuration.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum IoEngine {
    #[default]
    Sync,
    Async,
}
