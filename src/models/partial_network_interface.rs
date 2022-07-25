use serde::{Deserialize, Serialize};

use crate::models::RateLimiter;

/// Defines a partial network interface structure, used to update the rate limiters for that
/// interface, after microvm start.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartialNetworkInterface {
    pub iface_id: String,
    pub rx_rate_limiter: Option<Box<RateLimiter>>,
    pub tx_rate_limiter: Option<Box<RateLimiter>>,
}

impl PartialNetworkInterface {
    /// Defines a partial network interface structure, used to update the rate limiters for that
    /// interface, after microvm start.
    pub fn new(iface_id: String) -> Self {
        Self {
            iface_id,
            ..Default::default()
        }
    }
}
