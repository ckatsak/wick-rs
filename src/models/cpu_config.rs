use serde::{Deserialize, Serialize};

/// The CPU configuration template defines a set of bit maps as modifiers of flags accessed by
/// register to be disabled/enabled for the microvm.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CpuConfig {
    /// A collection of CPUIDs to be modified. (x86_64)
    #[serde(rename = "cpuid_modifiers", skip_serializing_if = "Option::is_none")]
    pub cpuid_modifiers: Option<serde_json::Value>,
    /// A collection of model specific registers to be modified. (x86_64)
    #[serde(rename = "msr_modifiers", skip_serializing_if = "Option::is_none")]
    pub msr_modifiers: Option<serde_json::Value>,
    /// A collection of registers to be modified. (aarch64)
    #[serde(rename = "reg_modifiers", skip_serializing_if = "Option::is_none")]
    pub reg_modifiers: Option<serde_json::Value>,
    /// A collection of vcpu features to be modified. (aarch64)
    #[serde(rename = "vcpu_features", skip_serializing_if = "Option::is_none")]
    pub vcpu_features: Option<serde_json::Value>,
    /// A collection of kvm capabilities to be modified. (aarch64)
    #[serde(rename = "kvm_capabilities", skip_serializing_if = "Option::is_none")]
    pub kvm_capabilities: Option<serde_json::Value>,
}

impl CpuConfig {
    /// The CPU configuration template defines a set of bit maps as modifiers of flags accessed by
    /// register to be disabled/enabled for the microvm.
    pub fn new() -> Self {
        Default::default()
    }
}
