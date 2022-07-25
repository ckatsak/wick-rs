use serde::{Deserialize, Serialize};

use crate::models::MemoryBackend;

/// Defines the configuration used for handling snapshot resume. Exactly one of the two `mem_*`
/// fields must be present in the body of the request.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SnapshotLoadParams {
    /// Enable support for incremental (diff) snapshots by tracking dirty guest pages.
    pub enable_diff_snapshots: Option<bool>,
    /// Path to the file that contains the guest memory to be loaded. This parameter has been
    /// deprecated and is only allowed if `mem_backend` is not present.
    pub mem_file_path: Option<String>,
    pub mem_backend: Option<Box<MemoryBackend>>,
    /// Path to the file that contains the microVM state to be loaded.
    pub snapshot_path: String,
    /// When set to true, the vm is also resumed if the snapshot load is successful.
    pub resume_vm: Option<bool>,
}

impl SnapshotLoadParams {
    /// Defines the configuration used for handling snapshot resume. Exactly one of the two `mem_*`
    /// fields must be present in the body of the request.
    pub fn new(snapshot_path: String) -> Self {
        Self {
            snapshot_path,
            ..Default::default()
        }
    }
}
