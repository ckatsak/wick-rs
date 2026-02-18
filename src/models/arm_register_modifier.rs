use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Modifier for an ARM (`aarch64`) register.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArmRegisterModifier {
    /// 64-bit register address as hex, binary, or decimal string (e.g., `\"0x0\"`, `\"0b0\"`,
    /// `\"0\"`).
    pub addr: CompactString,
    /// 128-bit bitmap string defining which bits to modify. Format is `\"0b\"` followed
    /// by up to 128 characters where:
    /// - `'0'` = clear bit,
    /// - `'1'` = set bit,
    /// - `'x'` = don't modify.
    ///
    /// Underscores can be used for readability.
    ///
    /// Example: `\"0b0000000000000000000000000000000000000000000000000000000000000001\"`
    pub bitmap: String,
}

impl ArmRegisterModifier {
    pub fn new(addr: impl Into<CompactString>, bitmap: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
            bitmap: bitmap.into(),
        }
    }
}
