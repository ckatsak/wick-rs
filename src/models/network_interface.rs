//use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::models::RateLimiter;

/// Defines a network interface.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub guest_mac: Option<String>,
    /// Host level path for the guest network interface
    pub host_dev_name: String,
    pub iface_id: String,
    pub rx_rate_limiter: Option<RateLimiter>,
    pub tx_rate_limiter: Option<RateLimiter>,
}

impl NetworkInterface {
    /// Defines a network interface.
    pub fn builder(host_dev_name: String, iface_id: String) -> NetworkInterfaceBuilder {
        NetworkInterfaceBuilder {
            guest_mac: None,
            host_dev_name,
            iface_id,
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        }
    }

    //pub fn guest_mac(&self) -> Option<Cow<'_, str>> {
    //    self.guest_mac.as_ref().map(|s| Cow::Borrowed(s.as_ref()))
    //}

    ///// Host level path for the guest network interface
    //pub fn host_dev_name(&self) -> Cow<'_, str> {
    //    Cow::Borrowed(&self.host_dev_name)
    //}

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

#[derive(Debug, Clone)]
pub struct NetworkInterfaceBuilder {
    guest_mac: Option<String>,
    host_dev_name: String,
    iface_id: String,
    rx_rate_limiter: Option<RateLimiter>,
    tx_rate_limiter: Option<RateLimiter>,
}

impl NetworkInterfaceBuilder {
    pub fn guest_mac(&mut self, guest_mac: impl AsRef<str>) -> &mut Self {
        self.guest_mac = Some(guest_mac.as_ref().to_owned());
        self
    }

    pub fn rx_rate_limiter(&mut self, rx_rate_limiter: RateLimiter) -> &mut Self {
        self.rx_rate_limiter = Some(rx_rate_limiter);
        self
    }

    pub fn tx_rate_limiter(&mut self, tx_rate_limiter: RateLimiter) -> &mut Self {
        self.tx_rate_limiter = Some(tx_rate_limiter);
        self
    }

    pub fn build(self) -> NetworkInterface {
        NetworkInterface {
            guest_mac: self.guest_mac,
            host_dev_name: self.host_dev_name,
            iface_id: self.iface_id,
            rx_rate_limiter: self.rx_rate_limiter,
            tx_rate_limiter: self.tx_rate_limiter,
        }
    }
}
