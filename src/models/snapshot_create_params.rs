use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SnapshotCreateParams {
    /// Path to the file that will contain the guest memory.
    pub mem_file_path: String,
    /// Path to the file that will contain the microVM state.
    pub snapshot_path: String,
    /// Type of snapshot to create. It is optional and by default, a full snapshot is created.
    pub snapshot_type: Option<SnapshotType>,
    /// The microVM version for which we want to create the snapshot. It is optional and it
    /// defaults to the current version.
    pub version: Option<String>,
}

impl SnapshotCreateParams {
    pub fn new(mem_file_path: String, snapshot_path: String) -> Self {
        Self {
            mem_file_path,
            snapshot_path,
            ..Default::default()
        }
    }
}

/// Type of snapshot to create. It is optional and by default, a full snapshot is created.
#[derive(
    Clone, Copy, Debug, Eq, Default, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum SnapshotType {
    #[default]
    Full,
    Diff,
}
