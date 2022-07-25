use serde::{Deserialize, Serialize};

use crate::models::RateLimiter;

/// Defines a network interface.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub guest_mac: Option<String>,
    /// Host level path for the guest network interface
    pub host_dev_name: String,
    pub iface_id: String,
    pub rx_rate_limiter: Option<Box<RateLimiter>>,
    pub tx_rate_limiter: Option<Box<RateLimiter>>,
}

impl NetworkInterface {
    /// Defines a network interface.
    pub fn new(host_dev_name: String, iface_id: String) -> Self {
        Self {
            host_dev_name,
            iface_id,
            ..Default::default()
        }
    }
}
