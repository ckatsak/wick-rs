use serde::{Deserialize, Serialize};

/// Describes MicroVM instance information.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstanceInfo {
    /// Application name.
    pub app_name: String,
    /// MicroVM / instance ID.
    pub id: String,
    /// The current detailed state (Not started, Running, Paused) of the Firecracker instance. This
    /// value is read-only for the control-plane.
    pub state: State,
    /// MicroVM hypervisor build version.
    pub vmm_version: String,
}

impl InstanceInfo {
    /// Describes MicroVM instance information.
    pub fn new(app_name: String, id: String, state: State, vmm_version: String) -> Self {
        Self {
            app_name,
            id,
            state,
            vmm_version,
        }
    }
}

/// The current detailed state (Not started, Running, Paused) of the Firecracker instance. This
/// value is read-only for the control-plane.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum State {
    #[default]
    #[serde(rename = "Not started")]
    NotStarted,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Paused")]
    Paused,
}
