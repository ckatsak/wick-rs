use serde::{Deserialize, Serialize};

use crate::models;

/// Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined by
/// configuring each of the _bandwidth_ and _ops_ token buckets. This field is optional for
/// virtio-block config and should be omitted for vhost-user-block configuration.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RateLimiter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<Box<models::TokenBucket>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops: Option<Box<models::TokenBucket>>,
}
