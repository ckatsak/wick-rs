use crate::models;
use serde::{Deserialize, Serialize};

/// Defines the configuration used for handling snapshot resume. Exactly one of the two `mem_*`
/// fields must be present in the body of the request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotLoadParams {
    /// Enable support for incremental (diff) snapshots by tracking dirty guest pages.
    #[serde(
        rename = "enable_diff_snapshots",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_diff_snapshots: Option<bool>,
    /// Path to the file that contains the guest memory to be loaded. It is only allowed if
    /// `mem_backend` is not present. This parameter has been deprecated and it will be removed
    /// in future Firecracker release.
    #[serde(rename = "mem_file_path", skip_serializing_if = "Option::is_none")]
    pub mem_file_path: Option<String>,
    #[serde(rename = "mem_backend", skip_serializing_if = "Option::is_none")]
    pub mem_backend: Option<Box<models::MemoryBackend>>,
    /// Path to the file that contains the microVM state to be loaded.
    #[serde(rename = "snapshot_path")]
    pub snapshot_path: String,
    /// When set to true, the vm is also resumed if the snapshot load is successful.
    #[serde(rename = "resume_vm", skip_serializing_if = "Option::is_none")]
    pub resume_vm: Option<bool>,
    /// Network host device names to override
    #[serde(rename = "network_overrides", skip_serializing_if = "Option::is_none")]
    pub network_overrides: Option<Vec<models::NetworkOverride>>,
}

impl SnapshotLoadParams {
    /// Defines the configuration used for handling snapshot resume. Exactly one of the two
    /// `mem_*` fields must be present in the body of the request.
    pub fn new(snapshot_path: String) -> Self {
        Self {
            enable_diff_snapshots: None,
            mem_file_path: None,
            mem_backend: None,
            snapshot_path,
            resume_vm: None,
            network_overrides: None,
        }
    }
}
