use serde::{Deserialize, Serialize};

use crate::models::TokenBucket;

/// Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined by
/// configuring each of the _bandwidth_ and _ops_ token buckets.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RateLimiter {
    pub bandwidth: Option<Box<TokenBucket>>,
    pub ops: Option<Box<TokenBucket>>,
}

impl RateLimiter {
    /// Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined by
    /// configuring each of the _bandwidth_ and _ops_ token buckets.
    pub fn new() -> Self {
        Default::default()
    }
}
