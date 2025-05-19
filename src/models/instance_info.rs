use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Describes MicroVM instance information.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InstanceInfo {
    /// Application name.
    pub app_name: CompactString,
    /// MicroVM / instance ID.
    pub id: CompactString,
    /// The current detailed state (Not started, Running, Paused) of the Firecracker instance.
    /// This value is read-only for the control-plane.
    pub state: State,
    /// MicroVM hypervisor build version.
    pub vmm_version: CompactString,
}

impl InstanceInfo {
    #[inline]
    pub fn new(
        app_name: impl Into<CompactString>,
        id: impl Into<CompactString>,
        state: State,
        vmm_version: impl Into<CompactString>,
    ) -> Self {
        Self {
            app_name: app_name.into(),
            id: id.into(),
            state,
            vmm_version: vmm_version.into(),
        }
    }
}

/// The current detailed state (Not started, Running, Paused) of the Firecracker instance.
/// This value is read-only for the control-plane.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum State {
    #[serde(rename = "Not started")]
    #[default]
    NotStarted,
    Running,
    Paused,
}
