use serde::{Deserialize, Serialize};

/// Update the statistics polling interval, with the first statistics update scheduled immediately.
/// Statistics cannot be turned on/off after boot.
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BalloonStatsUpdate {
    /// Interval in seconds between refreshing statistics.
    pub stats_polling_interval_s: i32,
}

impl BalloonStatsUpdate {
    /// Update the statistics polling interval, with the first statistics update scheduled
    /// immediately. Statistics cannot be turned on/off after boot.
    pub fn new(stats_polling_interval_s: i32) -> Self {
        Self {
            stats_polling_interval_s,
        }
    }
}
