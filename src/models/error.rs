use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    /// A description of the error condition.
    pub fault_message: Option<String>,
}

impl Error {
    pub fn new() -> Self {
        Default::default()
    }
}
