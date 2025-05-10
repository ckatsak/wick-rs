use serde::{Deserialize, Serialize};

/// Variant wrapper containing the real action.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceActionInfo {
    /// Enumeration indicating what type of action is contained in the payload
    #[serde(rename = "action_type")]
    pub action_type: ActionType,
}

impl InstanceActionInfo {
    /// Variant wrapper containing the real action.
    pub fn new(action_type: ActionType) -> Self {
        Self { action_type }
    }
}

/// Enumeration indicating what type of action is contained in the payload
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ActionType {
    #[serde(rename = "FlushMetrics")]
    #[default]
    FlushMetrics,
    #[serde(rename = "InstanceStart")]
    InstanceStart,
    #[serde(rename = "SendCtrlAltDel")]
    SendCtrlAltDel,
}
