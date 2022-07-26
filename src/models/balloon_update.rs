use serde::{Deserialize, Serialize};

/// Balloon device descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BalloonUpdate {
    /// Target balloon size in MiB.
    pub amount_mib: i32,
}

impl BalloonUpdate {
    /// Balloon device descriptor.
    pub fn new(amount_mib: i32) -> Self {
        Self { amount_mib }
    }
}
