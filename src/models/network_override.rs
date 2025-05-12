use serde::{Deserialize, Serialize};

/// Allows for changing the backing TAP device of a network interface during snapshot restore.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkOverride {
    /// The name of the interface to modify
    #[serde(rename = "iface_id")]
    pub iface_id: String,
    /// The new host device of the interface
    #[serde(rename = "host_dev_name")]
    pub host_dev_name: String,
}

impl NetworkOverride {
    /// Allows for changing the backing TAP device of a network interface during snapshot restore.
    pub fn new(iface_id: String, host_dev_name: String) -> Self {
        Self {
            iface_id,
            host_dev_name,
        }
    }
}
