use serde::{Deserialize, Serialize};

/// Defines the microVM running state. It is especially useful in the snapshotting context.
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Vm {
    pub state: State,
}

impl Vm {
    /// Defines the microVM running state. It is especially useful in the snapshotting context.
    pub fn new(state: State) -> Self {
        Self { state }
    }
}

#[derive(
    Clone, Copy, Debug, Eq, Default, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum State {
    #[default]
    Paused,
    Resumed,
}
