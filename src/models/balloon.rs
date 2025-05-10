use serde::{Deserialize, Serialize};

/// Balloon device descriptor.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Balloon {
    /// Target balloon size in MiB.
    #[serde(rename = "amount_mib")]
    pub amount_mib: i32,
    /// Whether the balloon should deflate when the guest has memory pressure.
    #[serde(rename = "deflate_on_oom")]
    pub deflate_on_oom: bool,
    /// Interval in seconds between refreshing statistics. A non-zero value will enable the
    /// statistics. Defaults to 0.
    #[serde(
        rename = "stats_polling_interval_s",
        skip_serializing_if = "Option::is_none"
    )]
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
}
