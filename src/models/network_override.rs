use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Allows for changing the backing TAP device of a network interface during snapshot restore.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NetworkOverride {
    /// The name of the interface to modify
    pub iface_id: CompactString,
    /// The new host device of the interface
    pub host_dev_name: CompactString,
}

impl NetworkOverride {
    #[inline]
    pub fn new(
        iface_id: impl Into<CompactString>,
        host_dev_name: impl Into<CompactString>,
    ) -> Self {
        Self {
            iface_id: iface_id.into(),
            host_dev_name: host_dev_name.into(),
        }
    }
}
