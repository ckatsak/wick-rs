use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Modifier for a model specific register (x86_64)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MsrModifier {
    /// 32-bit MSR address as hex, binary, or decimal string (e.g., `\"0x10a\"`, `\"0b100001010\"`,
    /// `\"266\"`)
    pub addr: CompactString,
    /// 64-bit bitmap string defining which bits to modify. Format is `\"0b\"` followed by 64
    /// characters where
    /// - `'0'` = clear bit,
    /// - `'1'` = set bit,
    /// - `'x'` = don't modify.
    ///
    /// Underscores can be used for readability.
    ///
    /// Example `\"0b0000000000000000000000000000000000000000000000000000000000000001\"`
    pub bitmap: String,
}

impl MsrModifier {
    /// Modifier for a model specific register (x86_64)
    pub fn new(addr: impl Into<CompactString>, bitmap: String) -> Self {
        Self {
            addr: addr.into(),
            bitmap,
        }
    }
}
