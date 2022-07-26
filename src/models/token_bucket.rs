use serde::{Deserialize, Serialize};

/// Defines a token bucket with a maximum capacity (`size`), an initial burst size
/// (`one_time_burst`) and an interval for refilling purposes (`refill_time`). The refill-rate
/// is derived from `size` and `refill_time`, and it is the constant rate at which the tokens
/// replenish. The refill process only starts happening after the initial burst budget is
/// consumed. Consumption from the token bucket is unbounded in speed which allows for bursts
/// bound in size by the amount of tokens available. Once the token bucket is empty,
/// consumption speed is bound by the `refill_rate`.
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TokenBucket {
    /// The initial size of a token bucket.
    pub one_time_burst: Option<i64>,
    /// The amount of milliseconds it takes for the bucket to refill.
    pub refill_time: i64,
    /// The total number of tokens this bucket can hold.
    pub size: i64,
}

impl TokenBucket {
    /// Defines a token bucket with a maximum capacity (`size`), an initial burst size
    /// (`one_time_burst`) and an interval for refilling purposes (`refill_time`). The refill-rate
    /// is derived from `size` and `refill_time`, and it is the constant rate at which the tokens
    /// replenish. The refill process only starts happening after the initial burst budget is
    /// consumed. Consumption from the token bucket is unbounded in speed which allows for bursts
    /// bound in size by the amount of tokens available. Once the token bucket is empty,
    /// consumption speed is bound by the `refill_rate`.
    pub fn new(refill_time: i64, size: i64) -> Self {
        Self {
            one_time_burst: None,
            refill_time,
            size,
        }
    }

    /// Defines a token bucket with a maximum capacity (`size`), an initial burst size
    /// (`one_time_burst`) and an interval for refilling purposes (`refill_time`). The refill-rate
    /// is derived from `size` and `refill_time`, and it is the constant rate at which the tokens
    /// replenish. The refill process only starts happening after the initial burst budget is
    /// consumed. Consumption from the token bucket is unbounded in speed which allows for bursts
    /// bound in size by the amount of tokens available. Once the token bucket is empty,
    /// consumption speed is bound by the `refill_rate`.
    pub fn with_one_time_burst(refill_time: i64, size: i64, one_time_burst: i64) -> Self {
        Self {
            one_time_burst: Some(one_time_burst),
            refill_time,
            size,
        }
    }

    ///// The initial size of a token bucket.
    //pub fn one_time_burst(&self) -> Option<i64> {
    //    self.one_time_burst
    //}

    ///// The amount of milliseconds it takes for the bucket to refill.
    //pub fn refill_time(&self) -> i64 {
    //    self.refill_time
    //}

    ///// The total number of tokens this bucket can hold.
    //pub fn size(&self) -> i64 {
    //    self.size
    //}
}
