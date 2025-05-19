use serde::{Deserialize, Serialize};

/// Balloon device descriptor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Balloon {
    /// Target balloon size in MiB.
    pub amount_mib: i32,
    /// Whether the balloon should deflate when the guest has memory pressure.
    pub deflate_on_oom: bool,
    /// Interval in seconds between refreshing statistics. A non-zero value will enable the
    /// statistics. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_polling_interval_s: Option<i32>,
}

impl Balloon {
    #[inline]
    pub fn new(amount_mib: i32, deflate_on_oom: bool) -> Self {
        Self {
            amount_mib,
            deflate_on_oom,
            stats_polling_interval_s: None,
        }
    }
}
