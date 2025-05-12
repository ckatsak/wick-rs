use serde::{Deserialize, Serialize};

/// Describes the Firecracker version.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirecrackerVersion {
    /// Firecracker build version.
    #[serde(rename = "firecracker_version")]
    pub firecracker_version: String,
}

impl FirecrackerVersion {
    /// Describes the Firecracker version.
    pub fn new(firecracker_version: String) -> Self {
        Self {
            firecracker_version,
        }
    }
}
