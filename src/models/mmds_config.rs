use serde::{Deserialize, Serialize};

/// Defines the MMDS configuration.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MmdsConfig {
    /// Enumeration indicating the MMDS version to be configured.
    pub version: Option<Version>,
    /// List of the network interface IDs capable of forwarding packets to the MMDS. Network
    /// interface IDs mentioned must be valid at the time of this request. The net device model
    /// will reply to HTTP GET requests sent to the MMDS address via the interfaces mentioned. In
    /// this case, both ARP requests and TCP segments heading to `ipv4_address` are intercepted by
    /// the device model, and do not reach the associated TAP device.
    pub network_interfaces: Vec<String>,
    /// A valid IPv4 link-local address.
    pub ipv4_address: Option<String>,
}

impl MmdsConfig {
    /// Defines the MMDS configuration.
    pub fn new(network_interfaces: Vec<String>) -> Self {
        Self {
            network_interfaces,
            ..Default::default()
        }
    }
}

/// Enumeration indicating the MMDS version to be configured.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Version {
    #[default]
    V1,
    V2,
}
