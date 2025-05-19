use serde::{Deserialize, Serialize};

use crate::models;

/// Describes the number of vCPUs, memory size, SMT capabilities, huge page configuration and the
/// CPU template.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MachineConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_template: Option<models::CpuTemplate>,
    /// Flag for enabling/disabling simultaneous multithreading. Can be enabled only on x86.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smt: Option<bool>,
    /// Memory size of VM
    pub mem_size_mib: i32,
    /// Enable dirty page tracking. If this is enabled, then incremental guest memory snapshots
    /// can be created. These belong to diff snapshots, which contain, besides the microVM state,
    /// only the memory dirtied since a previous snapshot. Full snapshots each contain a full copy
    /// of the guest memory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_dirty_pages: Option<bool>,
    /// Number of vCPUs (either 1 or an even number)
    pub vcpu_count: i32,
    /// Which huge pages configuration (if any) should be used to back guest memory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub huge_pages: Option<HugePages>,
}

impl MachineConfiguration {
    #[inline]
    pub fn new(mem_size_mib: i32, vcpu_count: i32) -> Self {
        Self {
            cpu_template: None,
            smt: None,
            mem_size_mib,
            track_dirty_pages: None,
            vcpu_count,
            huge_pages: None,
        }
    }
}

/// Which huge pages configuration (if any) should be used to back guest memory.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum HugePages {
    #[default]
    None,
    #[serde(rename = "2M")]
    TwoM,
}
