use std::path::{Path, PathBuf};

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
    /// Host level path for the guest drive.
    pub path_on_host: PathBuf,
    pub rate_limiter: Option<RateLimiter>,
    /// Type of the IO engine used by the device. `Async` is supported on host kernels newer than
    /// 5.10.51.
    pub io_engine: Option<IoEngine>,
}

impl Drive {
    pub fn builder(drive_id: String, path_on_host: impl AsRef<Path>) -> DriveBuilder {
        DriveBuilder {
            drive_id,
            cache_type: None,
            is_read_only: false,
            is_root_device: false,
            partuuid: None,
            path_on_host: path_on_host.as_ref().to_path_buf(),
            rate_limiter: None,
            io_engine: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DriveBuilder {
    drive_id: String,
    cache_type: Option<CacheType>,
    is_read_only: bool,
    is_root_device: bool,
    partuuid: Option<String>,
    path_on_host: PathBuf,
    rate_limiter: Option<RateLimiter>,
    io_engine: Option<IoEngine>,
}

impl DriveBuilder {
    pub fn cache_type(&mut self, cache_type: CacheType) -> &mut Self {
        self.cache_type = Some(cache_type);
        self
    }

    pub fn is_read_only(&mut self, is_read_only: bool) -> &mut Self {
        self.is_read_only = is_read_only;
        self
    }

    pub fn is_root_device(&mut self, is_root_device: bool) -> &mut Self {
        self.is_root_device = is_root_device;
        self
    }

    pub fn partuuid(&mut self, partuuid: impl AsRef<str>) -> &mut Self {
        self.partuuid = Some(partuuid.as_ref().to_owned());
        self
    }

    pub fn rate_limiter(&mut self, rate_limiter: RateLimiter) -> &mut Self {
        self.rate_limiter = Some(rate_limiter);
        self
    }

    pub fn io_engine(&mut self, io_engine: IoEngine) -> &mut Self {
        self.io_engine = Some(io_engine);
        self
    }

    pub fn build(self) -> Drive {
        Drive {
            drive_id: self.drive_id,
            cache_type: self.cache_type,
            is_read_only: self.is_read_only,
            is_root_device: self.is_root_device,
            partuuid: self.partuuid,
            path_on_host: self.path_on_host,
            rate_limiter: self.rate_limiter,
            io_engine: self.io_engine,
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
