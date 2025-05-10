use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    /// A description of the error condition
    #[serde(rename = "fault_message", skip_serializing_if = "Option::is_none")]
    pub fault_message: Option<String>,
}

impl Error {
    pub fn new() -> Self {
        Default::default()
    }
}
