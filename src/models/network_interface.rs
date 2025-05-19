use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::models;

/// Defines a network interface.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterface {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_mac: Option<CompactString>,
    /// Host level path for the guest network interface
    pub host_dev_name: CompactString,
    pub iface_id: CompactString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_rate_limiter: Option<Box<models::RateLimiter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rate_limiter: Option<Box<models::RateLimiter>>,
}

impl NetworkInterface {
    #[inline]
    pub fn new(
        host_dev_name: impl Into<CompactString>,
        iface_id: impl Into<CompactString>,
    ) -> Self {
        Self {
            guest_mac: None,
            host_dev_name: host_dev_name.into(),
            iface_id: iface_id.into(),
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        }
    }
}
