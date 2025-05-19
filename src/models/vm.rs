use serde::{Deserialize, Serialize};

/// Defines the microVM running state. It is especially useful in the snapshotting context.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vm {
    pub state: State,
}

impl Vm {
    #[inline]
    pub fn new(state: State) -> Self {
        Self { state }
    }
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum State {
    #[default]
    Paused,
    Resumed,
}
