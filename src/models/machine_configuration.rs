use serde::{Deserialize, Serialize};

use crate::models::CpuTemplate;

/// Describes the number of vCPUs, memory size, SMT capabilities and the CPU template.
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
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
    pub fn builder(mem_size_mib: i32, vcpu_count: i32) -> MachineConfigurationBuilder {
        MachineConfigurationBuilder {
            cpu_template: None,
            smt: None,
            mem_size_mib,
            track_dirty_pages: None,
            vcpu_count,
        }
    }

    //pub fn cpu_template(&self) -> Option<CpuTemplate> {
    //    self.cpu_template
    //}

    ///// Flag for enabling/disabling simultaneous multithreading. Can be enabled only on x86.
    //pub fn smt(&self) -> Option<bool> {
    //    self.smt
    //}

    ///// Memory size of VM
    //pub fn mem_size_mib(&self) -> i32 {
    //    self.mem_size_mib
    //}

    ///// Enable dirty page tracking. If this is enabled, then incremental guest memory snapshots can
    ///// be created. These belong to diff snapshots, which contain, besides the microVM state, only
    ///// the memory dirtied since a previous snapshot. Full snapshots each contain a full copy of
    ///// the guest memory.
    //pub fn track_dirty_pages(&self) -> Option<bool> {
    //    self.track_dirty_pages
    //}

    ///// Number of vCPUs (either 1 or an even number)
    //pub fn vcpu_count(&self) -> i32 {
    //    self.vcpu_count
    //}
}

#[derive(Debug, Clone, Copy)]
pub struct MachineConfigurationBuilder {
    cpu_template: Option<CpuTemplate>,
    smt: Option<bool>,
    mem_size_mib: i32,
    track_dirty_pages: Option<bool>,
    vcpu_count: i32,
}

impl MachineConfigurationBuilder {
    pub fn cpu_template(&mut self, cpu_template: CpuTemplate) -> &mut Self {
        self.cpu_template = Some(cpu_template);
        self
    }

    /// Flag for enabling/disabling simultaneous multithreading. Can be enabled only on x86.
    pub fn smt(&mut self, smt: bool) -> &mut Self {
        self.smt = Some(smt);
        self
    }

    /// Enable dirty page tracking. If this is enabled, then incremental guest memory snapshots can
    /// be created. These belong to diff snapshots, which contain, besides the microVM state, only
    /// the memory dirtied since a previous snapshot. Full snapshots each contain a full copy of
    /// the guest memory.
    pub fn track_dirty_pages(&mut self, track_dirty_pages: bool) -> &mut Self {
        self.track_dirty_pages = Some(track_dirty_pages);
        self
    }

    pub fn build(self) -> MachineConfiguration {
        MachineConfiguration {
            cpu_template: self.cpu_template,
            smt: self.smt,
            mem_size_mib: self.mem_size_mib,
            track_dirty_pages: self.track_dirty_pages,
            vcpu_count: self.vcpu_count,
        }
    }
}
