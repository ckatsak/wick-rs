use serde::{Deserialize, Serialize};

/// Balloon device descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Balloon {
    /// Target balloon size in MiB.
    pub amount_mib: i32,
    /// Whether the balloon should deflate when the guest has memory pressure.
    pub deflate_on_oom: bool,
    /// Interval in seconds between refreshing statistics. A non-zero value will enable the
    /// statistics. Defaults to 0.
    pub stats_polling_interval_s: Option<i32>,
}

impl Balloon {
    /// Balloon device descriptor.
    pub fn new(amount_mib: i32, deflate_on_oom: bool) -> Self {
        Self {
            amount_mib,
            deflate_on_oom,
            stats_polling_interval_s: None,
        }
    }

    /// Balloon device descriptor.
    pub fn with_stats_polling_interval(
        amount_mib: i32,
        deflate_on_oom: bool,
        stats_polling_interval_s: i32,
    ) -> Self {
        Self {
            amount_mib,
            deflate_on_oom,
            stats_polling_interval_s: Some(stats_polling_interval_s),
        }
    }

    ///// Target balloon size in MiB.
    //pub fn amount_mib(&self) -> i32 {
    //    self.amount_mib
    //}

    ///// Whether the balloon should deflate when the guest has memory pressure.
    //pub fn deflate_on_oom(&self) -> bool {
    //    self.deflate_on_oom
    //}

    ///// Interval in seconds between refreshing statistics. A non-zero value will enable the
    ///// statistics. Defaults to 0.
    //pub fn stats_polling_interval_s(&self) -> Option<i32> {
    //    self.stats_polling_interval_s
    //}
}
