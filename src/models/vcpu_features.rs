use serde::{Deserialize, Serialize};

/// vCPU feature modifier (aarch64)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VcpuFeatures {
    /// Index in the `kvm_vcpu_init.features` array
    pub index: i32,
    /// 32-bit bitmap string defining which bits to modify. Format is `\"0b\"` followed by 32
    /// characters where
    /// - `'0'` = clear bit,
    /// - `'1'` = set bit,
    /// - `'x'` = don't modify.
    ///
    /// Example: `\"0b00000000000000000000000001100000\"`
    pub bitmap: String,
}

impl VcpuFeatures {
    /// vCPU feature modifier (aarch64)
    pub fn new(index: i32, bitmap: String) -> Self {
        Self { index, bitmap }
    }
}
