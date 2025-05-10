use serde::{Deserialize, Serialize};

/// Balloon device descriptor.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BalloonUpdate {
    /// Target balloon size in MiB.
    #[serde(rename = "amount_mib")]
    pub amount_mib: i32,
}

impl BalloonUpdate {
    /// Balloon device descriptor.
    pub fn new(amount_mib: i32) -> Self {
        Self { amount_mib }
    }
}
