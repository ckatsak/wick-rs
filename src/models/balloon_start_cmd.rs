use serde::{Deserialize, Serialize};

/// Command used to start a free page hinting run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BalloonStartCmd {
    /// If Firecracker should automatically acknowledge when the guest submits a done cmd.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledge_on_stop: Option<bool>,
}
