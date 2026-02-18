use serde::{Deserialize, Serialize};

/// Modifier for a specific CPUID register within a leaf (x86_64)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CpuidRegisterModifier {
    /// Target CPUID register name
    pub register: Register,
    /// 32-bit bitmap string defining which bits to modify. Format is `\"0b\"` followed by 32
    /// characters where:
    /// - `'0'` = clear bit,
    /// - `'1'` = set bit,
    /// - `'x'` = don't modify.
    ///
    /// Example: `\"0b00000000000000000000000000000001\"` or
    /// `\"0bxxxxxxxxxxxxxxxxxxxxxxxxxxxx0001\"`
    pub bitmap: String,
}

impl CpuidRegisterModifier {
    /// Modifier for a specific CPUID register within a leaf (x86_64)
    pub fn new(register: Register, bitmap: impl Into<String>) -> Self {
        Self {
            register,
            bitmap: bitmap.into(),
        }
    }
}

/// Target CPUID register name
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Register {
    #[serde(rename = "eax")]
    #[default]
    Eax,
    #[serde(rename = "ebx")]
    Ebx,
    #[serde(rename = "ecx")]
    Ecx,
    #[serde(rename = "edx")]
    Edx,
}
