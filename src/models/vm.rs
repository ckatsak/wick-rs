use serde::{Deserialize, Serialize};

/// Defines the microVM running state. It is especially useful in the snapshotting context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vm {
    #[serde(rename = "state")]
    pub state: State,
}

impl Vm {
    /// Defines the microVM running state. It is especially useful in the snapshotting context.
    pub fn new(state: State) -> Self {
        Self { state }
    }
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum State {
    #[serde(rename = "Paused")]
    #[default]
    Paused,
    #[serde(rename = "Resumed")]
    Resumed,
}
