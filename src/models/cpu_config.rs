use serde::{Deserialize, Serialize};

use crate::models;

/// The CPU configuration template defines a set of bit maps as modifiers of flags accessed by
/// register to be disabled/enabled for the microvm.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CpuConfig {
    /// A collection of KVM capabilities to be added or removed (both x86_64 and aarch64).
    ///
    /// `Vec` item: KVM capability as a numeric string.
    /// Prefix with `'!'` to remove capability.
    /// Example: `"121"` (add) or `"!121"` (remove)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvm_capabilities: Option<Vec<String>>,
    /// A collection of CPUID leaf modifiers (x86_64 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuid_modifiers: Option<Vec<models::CpuidLeafModifier>>,
    /// A collection of model specific register modifiers (x86_64 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msr_modifiers: Option<Vec<models::MsrModifier>>,
    /// A collection of register modifiers (aarch64 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_modifiers: Option<Vec<models::ArmRegisterModifier>>,
    /// A collection of vCPU features to be modified (aarch64 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu_features: Option<Vec<models::VcpuFeatures>>,
}
