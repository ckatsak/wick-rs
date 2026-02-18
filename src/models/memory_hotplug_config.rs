use serde::{Deserialize, Serialize};

/// The configuration of the hotpluggable memory device (virtio-mem)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MemoryHotplugConfig {
    /// Total size of the hotpluggable memory in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_mib: Option<i32>,
    /// Slot size for the hotpluggable memory in MiB. This will determine the granularity of
    /// hot-plug memory from the host. Refer to the device documentation on how to tune this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_size_mib: Option<i32>,
    /// (Logical) Block size for the hotpluggable memory in MiB. This will determine the logical
    /// granularity of hot-plug memory for the guest. Refer to the device documentation on how to
    /// tune this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size_mib: Option<i32>,
}
