use crate::models;
use serde::{Deserialize, Serialize};

/// Defines a partial network interface structure, used to update the rate limiters for that
/// interface, after microvm start.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialNetworkInterface {
    #[serde(rename = "iface_id")]
    pub iface_id: String,
    #[serde(rename = "rx_rate_limiter", skip_serializing_if = "Option::is_none")]
    pub rx_rate_limiter: Option<Box<models::RateLimiter>>,
    #[serde(rename = "tx_rate_limiter", skip_serializing_if = "Option::is_none")]
    pub tx_rate_limiter: Option<Box<models::RateLimiter>>,
}

impl PartialNetworkInterface {
    /// Defines a partial network interface structure, used to update the rate limiters for that
    /// interface, after microvm start.
    pub fn new(iface_id: String) -> Self {
        Self {
            iface_id,
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        }
    }
}
