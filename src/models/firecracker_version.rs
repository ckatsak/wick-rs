use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Describes the Firecracker version.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FirecrackerVersion {
    /// Firecracker build version.
    pub firecracker_version: CompactString,
}

impl FirecrackerVersion {
    #[inline]
    pub fn new(firecracker_version: impl Into<CompactString>) -> Self {
        Self {
            firecracker_version: firecracker_version.into(),
        }
    }
}
