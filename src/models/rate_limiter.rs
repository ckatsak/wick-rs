use crate::models;
use serde::{Deserialize, Serialize};

/// Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined by
/// configuring each of the _bandwidth_ and _ops_ token buckets. This field is optional for
/// virtio-block config and should be omitted for vhost-user-block configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RateLimiter {
    #[serde(rename = "bandwidth", skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<Box<models::TokenBucket>>,
    #[serde(rename = "ops", skip_serializing_if = "Option::is_none")]
    pub ops: Option<Box<models::TokenBucket>>,
}

impl RateLimiter {
    /// Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined
    /// by configuring each of the _bandwidth_ and _ops_ token buckets. This field is optional
    /// for virtio-block config and should be omitted for vhost-user-block configuration.
    pub fn new() -> Self {
        Self {
            bandwidth: None,
            ops: None,
        }
    }
}
