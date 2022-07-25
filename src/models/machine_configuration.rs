use serde::{Deserialize, Serialize};

use crate::models::CpuTemplate;

/// Describes the number of vCPUs, memory size, SMT capabilities and the CPU template.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MachineConfiguration {
    pub cpu_template: Option<CpuTemplate>,
    /// Flag for enabling/disabling simultaneous multithreading. Can be enabled only on x86.
    pub smt: Option<bool>,
    /// Memory size of VM
    pub mem_size_mib: i32,
    /// Enable dirty page tracking. If this is enabled, then incremental guest memory snapshots can
    /// be created. These belong to diff snapshots, which contain, besides the microVM state, only
    /// the memory dirtied since a previous snapshot. Full snapshots each contain a full copy of
    /// the guest memory.
    pub track_dirty_pages: Option<bool>,
    /// Number of vCPUs (either 1 or an even number)
    pub vcpu_count: i32,
}

impl MachineConfiguration {
    /// Describes the number of vCPUs, memory size, SMT capabilities and the CPU template.
    pub fn new(mem_size_mib: i32, vcpu_count: i32) -> Self {
        Self {
            mem_size_mib,
            vcpu_count,
            ..Default::default()
        }
    }
}
