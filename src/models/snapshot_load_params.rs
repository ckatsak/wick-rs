use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

use crate::models;

/// Defines the configuration used for handling snapshot resume. Exactly one of the two `mem_*`
/// fields must be present in the body of the request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SnapshotLoadParams {
    /// Enable dirty page tracking to improve space efficiency of diff snapshots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_dirty_pages: Option<bool>,
    /// Path to the file that contains the guest memory to be loaded. It is only allowed if
    /// `mem_backend` is not present. This parameter has been deprecated and it will be removed
    /// in future Firecracker release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_file_path: Option<Utf8PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_backend: Option<models::MemoryBackend>,
    /// Path to the file that contains the microVM state to be loaded.
    pub snapshot_path: Utf8PathBuf,
    /// When set to true, the vm is also resumed if the snapshot load is successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_vm: Option<bool>,
    /// Network host device names to override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_overrides: Option<Vec<models::NetworkOverride>>,
}

impl SnapshotLoadParams {
    #[inline]
    pub fn new(snapshot_path: impl Into<Utf8PathBuf>) -> Self {
        Self {
            track_dirty_pages: None,
            mem_file_path: None,
            mem_backend: None,
            snapshot_path: snapshot_path.into(),
            resume_vm: None,
            network_overrides: None,
        }
    }
}
