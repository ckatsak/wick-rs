//use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::models::RateLimiter;

/// Defines a partial network interface structure, used to update the rate limiters for that
/// interface, after microvm start.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartialNetworkInterface {
    pub iface_id: String,
    pub rx_rate_limiter: Option<RateLimiter>,
    pub tx_rate_limiter: Option<RateLimiter>,
}

impl PartialNetworkInterface {
    /// Defines a partial network interface structure, used to update the rate limiters for that
    /// interface, after microvm start.
    pub fn builder(iface_id: impl AsRef<str>) -> PartialNetworkInterfaceBuilder {
        PartialNetworkInterfaceBuilder {
            iface_id: iface_id.as_ref().to_owned(),
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        }
    }

    //pub fn iface_id(&self) -> Cow<'_, str> {
    //    Cow::Borrowed(&self.iface_id)
    //}

    //pub fn rx_rate_limiter(&self) -> Option<&RateLimiter> {
    //    self.rx_rate_limiter.as_ref()
    //}

    //pub fn tx_rate_limiter(&self) -> Option<&RateLimiter> {
    //    self.tx_rate_limiter.as_ref()
    //}
}

pub struct PartialNetworkInterfaceBuilder {
    iface_id: String,
    rx_rate_limiter: Option<RateLimiter>,
    tx_rate_limiter: Option<RateLimiter>,
}

impl PartialNetworkInterfaceBuilder {
    pub fn rx_rate_limiter(&mut self, rx_rate_limiter: RateLimiter) -> &mut Self {
        self.rx_rate_limiter = Some(rx_rate_limiter);
        self
    }

    pub fn tx_rate_limiter(&mut self, tx_rate_limiter: RateLimiter) -> &mut Self {
        self.tx_rate_limiter = Some(tx_rate_limiter);
        self
    }

    pub fn build(self) -> PartialNetworkInterface {
        PartialNetworkInterface {
            iface_id: self.iface_id,
            rx_rate_limiter: self.rx_rate_limiter,
            tx_rate_limiter: self.tx_rate_limiter,
        }
    }
}
