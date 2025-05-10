use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotCreateParams {
    /// Path to the file that will contain the guest memory.
    #[serde(rename = "mem_file_path")]
    pub mem_file_path: String,
    /// Path to the file that will contain the microVM state.
    #[serde(rename = "snapshot_path")]
    pub snapshot_path: String,
    /// Type of snapshot to create. It is optional and by default, a full snapshot is created.
    #[serde(rename = "snapshot_type", skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<SnapshotType>,
}

impl SnapshotCreateParams {
    pub fn new(mem_file_path: String, snapshot_path: String) -> Self {
        Self {
            mem_file_path,
            snapshot_path,
            snapshot_type: None,
        }
    }
}

/// Type of snapshot to create. It is optional and by default, a full snapshot is created.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum SnapshotType {
    #[serde(rename = "Full")]
    #[default]
    Full,
    #[serde(rename = "Diff")]
    Diff,
}
