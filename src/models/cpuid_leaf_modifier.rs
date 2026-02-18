use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::models;

/// Modifier for a CPUID leaf and subleaf (x86_64)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CpuidLeafModifier {
    /// CPUID leaf index as hex, binary, or decimal string (e.g., `\"0x0\"`, `\"0b0\"`, `\"0\"`)
    pub leaf: CompactString,
    /// CPUID subleaf index as hex, binary, or decimal string (e.g., `\"0x0\"`, `\"0b0\"`, `\"0\"`)
    pub subleaf: CompactString,
    /// KVM feature flags for this leaf-subleaf
    pub flags: i32,
    /// Register modifiers for this CPUID leaf
    pub modifiers: Vec<models::CpuidRegisterModifier>,
}

impl CpuidLeafModifier {
    pub fn new(
        leaf: impl Into<CompactString>,
        subleaf: impl Into<CompactString>,
        flags: i32,
        modifiers: Vec<models::CpuidRegisterModifier>,
    ) -> Self {
        Self {
            leaf: leaf.into(),
            subleaf: subleaf.into(),
            flags,
            modifiers,
        }
    }
}
