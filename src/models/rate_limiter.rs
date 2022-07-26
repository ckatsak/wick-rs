use serde::{Deserialize, Serialize};

use crate::models::TokenBucket;

/// Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined by
/// configuring each of the `bandwidth` and `ops` token buckets.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RateLimiter {
    pub bandwidth: Option<TokenBucket>,
    pub ops: Option<TokenBucket>,
}

impl RateLimiter {
    /// Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined by
    /// configuring each of the `bandwidth` and `ops` token buckets.
    pub fn builder() -> RateLimiterBuilder {
        RateLimiterBuilder {
            bandwidth: None,
            ops: None,
        }
    }

    //pub fn bandwidth(&self) -> Option<&TokenBucket> {
    //    self.bandwidth.as_ref()
    //}

    //pub fn ops(&self) -> Option<&TokenBucket> {
    //    self.ops.as_ref()
    //}
}

/// Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined by
/// configuring each of the `bandwidth` and `ops` token buckets.
pub struct RateLimiterBuilder {
    bandwidth: Option<TokenBucket>,
    ops: Option<TokenBucket>,
}

impl RateLimiterBuilder {
    pub fn bandwidth(&mut self, bandwidth: TokenBucket) -> &mut Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    pub fn ops(&mut self, ops: TokenBucket) -> &mut Self {
        self.ops = Some(ops);
        self
    }

    pub fn build(self) -> RateLimiter {
        RateLimiter {
            bandwidth: self.bandwidth,
            ops: self.ops,
        }
    }
}
