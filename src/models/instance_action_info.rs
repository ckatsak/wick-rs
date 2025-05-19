use serde::{Deserialize, Serialize};

/// Variant wrapper containing the real action.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InstanceActionInfo {
    /// Enumeration indicating what type of action is contained in the payload
    pub action_type: ActionType,
}

impl InstanceActionInfo {
    #[inline]
    pub fn new(action_type: ActionType) -> Self {
        Self { action_type }
    }
}

/// Enumeration indicating what type of action is contained in the payload
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ActionType {
    #[default]
    FlushMetrics,
    InstanceStart,
    SendCtrlAltDel,
}
