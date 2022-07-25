use serde::{Deserialize, Serialize};

/// The CPU Template defines a set of flags to be disabled from the microvm so that the features
/// exposed to the guest are the same as in the selected instance type. Works only on Intel.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum CpuTemplate {
    #[default]
    #[serde(rename = "C3")]
    C3,
    #[serde(rename = "T2")]
    T2,
    #[serde(rename = "None")]
    None,
}

impl ToString for CpuTemplate {
    fn to_string(&self) -> String {
        match self {
            Self::C3 => String::from("C3"),
            Self::T2 => String::from("T2"),
            Self::None => String::from("None"),
        }
    }
}
