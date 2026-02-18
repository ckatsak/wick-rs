use serde::{Deserialize, Serialize};

/// The status of the hotpluggable memory device (virtio-mem)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MemoryHotplugStatus {
    /// Total size of the hotpluggable memory in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_mib: Option<i32>,
    /// Slot size for the hotpluggable memory in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_size_mib: Option<i32>,
    /// (Logical) Block size for the hotpluggable memory in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size_mib: Option<i32>,
    /// Plugged size for the hotpluggable memory in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugged_size_mib: Option<i32>,
    /// Requested size for the hotpluggable memory in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_size_mib: Option<i32>,
}
