use crate::models;
use serde::{Deserialize, Serialize};

/// Defines a network interface.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterface {
    #[serde(rename = "guest_mac", skip_serializing_if = "Option::is_none")]
    pub guest_mac: Option<String>,
    /// Host level path for the guest network interface
    #[serde(rename = "host_dev_name")]
    pub host_dev_name: String,
    #[serde(rename = "iface_id")]
    pub iface_id: String,
    #[serde(rename = "rx_rate_limiter", skip_serializing_if = "Option::is_none")]
    pub rx_rate_limiter: Option<Box<models::RateLimiter>>,
    #[serde(rename = "tx_rate_limiter", skip_serializing_if = "Option::is_none")]
    pub tx_rate_limiter: Option<Box<models::RateLimiter>>,
}

impl NetworkInterface {
    /// Defines a network interface.
    pub fn new(host_dev_name: String, iface_id: String) -> Self {
        Self {
            guest_mac: None,
            host_dev_name,
            iface_id,
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        }
    }
}
