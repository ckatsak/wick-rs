use serde::{Deserialize, Serialize};

/// The CPU Template defines a set of flags to be disabled from the microvm so that the features
/// exposed to the guest are the same as in the selected instance type. This parameter has been
/// deprecated and it will be removed in future Firecracker release.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum CpuTemplate {
    #[default]
    C3,
    T2,
    T2S,
    T2CL,
    T2A,
    V1N1,
    None,
}

impl ::std::fmt::Display for CpuTemplate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Self::C3 => write!(f, "C3"),
            Self::T2 => write!(f, "T2"),
            Self::T2S => write!(f, "T2S"),
            Self::T2CL => write!(f, "T2CL"),
            Self::T2A => write!(f, "T2A"),
            Self::V1N1 => write!(f, "V1N1"),
            Self::None => write!(f, "None"),
        }
    }
}
