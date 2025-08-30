use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Defines the MMDS configuration.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MmdsConfig {
    /// Enumeration indicating the MMDS version to be configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
    /// List of the network interface IDs capable of forwarding packets to the MMDS. Network
    /// interface IDs mentioned must be valid at the time of this request. The net device model
    /// will reply to HTTP GET requests sent to the MMDS address via the interfaces mentioned.
    /// In this case, both ARP requests and TCP segments heading to `ipv4_address` are intercepted
    /// by the device model, and do not reach the associated TAP device.
    pub network_interfaces: Vec<CompactString>,
    /// A valid IPv4 link-local address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<CompactString>,
    /// MMDS operates compatibly with EC2 IMDS (i.e. responds "text/plain" content regardless of
    /// `Accept` header in requests).
    pub imds_comat: bool,
}

impl MmdsConfig {
    #[inline]
    pub fn new(network_interfaces: Vec<CompactString>) -> Self {
        Self {
            version: None,
            network_interfaces,
            ipv4_address: None,
            imds_comat: false,
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
