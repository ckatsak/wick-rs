use serde::{Deserialize, Serialize};

/// Describes the free page hinting status.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BalloonHintingStatus {
    /// The last command issued by the host.
    pub host_cmd: i32,
    /// The last command provided by the guest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_cmd: Option<i32>,
}

impl BalloonHintingStatus {
    pub fn new(host_cmd: i32) -> Self {
        Self {
            host_cmd,
            guest_cmd: None,
        }
    }
}
