use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SnapshotCreateParams {
    /// Path to the file that will contain the guest memory.
    pub mem_file_path: Utf8PathBuf,
    /// Path to the file that will contain the microVM state.
    pub snapshot_path: Utf8PathBuf,
    /// Type of snapshot to create. It is optional and by default, a full snapshot is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<SnapshotType>,
}

impl SnapshotCreateParams {
    #[inline]
    pub fn new(
        mem_file_path: impl Into<Utf8PathBuf>,
        snapshot_path: impl Into<Utf8PathBuf>,
    ) -> Self {
        Self {
            mem_file_path: mem_file_path.into(),
            snapshot_path: snapshot_path.into(),
            snapshot_type: None,
        }
    }
}

/// Type of snapshot to create. It is optional and by default, a full snapshot is created.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum SnapshotType {
    #[default]
    Full,
    Diff,
}
