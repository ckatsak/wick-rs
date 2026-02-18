use serde::{Deserialize, Serialize};

/// An update to the size of the hotpluggable memory region.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MemoryHotplugSizeUpdate {
    /// New target region size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_size_mib: Option<i32>,
}
