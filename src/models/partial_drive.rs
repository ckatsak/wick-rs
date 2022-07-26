use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::models::RateLimiter;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartialDrive {
    pub drive_id: String,
    /// Host level path for the guest drive
    pub path_on_host: Option<PathBuf>,
    pub rate_limiter: Option<RateLimiter>,
}

impl PartialDrive {
    pub fn builder(drive_id: String) -> PartialDriveBuilder {
        PartialDriveBuilder {
            drive_id,
            path_on_host: None,
            rate_limiter: None,
        }
    }

    //pub fn drive_id(&self) -> Cow<'_, str> {
    //    Cow::Borrowed(&self.drive_id)
    //}

    ///// Host level path for the guest drive
    //pub fn path_on_host(&self) -> Option<Cow<'_, Path>> {
    //    self.path_on_host
    //        .as_ref()
    //        .map(|p| Cow::Borrowed(p.as_ref()))
    //}

    //pub fn rate_limiter(&self) -> Option<&RateLimiter> {
    //    self.rate_limiter.as_ref()
    //}
}

#[derive(Clone, Debug)]
pub struct PartialDriveBuilder {
    drive_id: String,
    path_on_host: Option<PathBuf>,
    rate_limiter: Option<RateLimiter>,
}

impl PartialDriveBuilder {
    /// Host level path for the guest drive
    pub fn path_on_host(&mut self, path_on_host: impl AsRef<Path>) -> &mut Self {
        self.path_on_host = Some(path_on_host.as_ref().to_path_buf());
        self
    }

    pub fn rate_limiter(&mut self, rate_limiter: RateLimiter) -> &mut Self {
        self.rate_limiter = Some(rate_limiter);
        self
    }

    pub fn build(self) -> PartialDrive {
        PartialDrive {
            drive_id: self.drive_id,
            path_on_host: self.path_on_host,
            rate_limiter: self.rate_limiter,
        }
    }
}
