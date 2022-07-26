use serde::{Deserialize, Serialize};

/// Variant wrapper containing the real action.
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstanceActionInfo {
    pub action_type: ActionType,
}

impl InstanceActionInfo {
    /// Variant wrapper containing the real action.
    pub fn new(action_type: ActionType) -> Self {
        Self { action_type }
    }

    ///// Enumeration indicating what type of action is contained in the payload.
    //pub fn action_type(&self) -> ActionType {
    //    self.action_type
    //}
}

/// Enumeration indicating what type of action is contained in the payload.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ActionType {
    #[default]
    FlushMetrics,
    InstanceStart,
    SendCtrlAltDel,
}
