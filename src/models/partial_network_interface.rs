use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::models;

/// Defines a partial network interface structure, used to update the rate limiters for that
/// interface, after microvm start.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PartialNetworkInterface {
    pub iface_id: CompactString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_rate_limiter: Option<Box<models::RateLimiter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rate_limiter: Option<Box<models::RateLimiter>>,
}

impl PartialNetworkInterface {
    #[inline]
    pub fn new(iface_id: impl Into<CompactString>) -> Self {
        Self {
            iface_id: iface_id.into(),
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        }
    }
}
