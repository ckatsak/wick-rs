use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Error {
    /// A description of the error condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_message: Option<String>,
}
