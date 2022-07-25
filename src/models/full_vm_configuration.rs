use serde::{Deserialize, Serialize};

use crate::models::{
    Balloon, BootSource, Drive, Logger, MachineConfiguration, Metrics, MmdsConfig,
    NetworkInterface, Vsock,
};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FullVmConfiguration {
    pub balloon: Option<Box<Balloon>>,
    /// Configurations for all block devices.
    pub drives: Option<Vec<Drive>>,
    #[serde(rename = "boot-source")]
    pub boot_source: Option<Box<BootSource>>,
    pub logger: Option<Box<Logger>>,
    #[serde(rename = "machine-config")]
    pub machine_config: Option<Box<MachineConfiguration>>,
    pub metrics: Option<Box<Metrics>>,
    #[serde(rename = "mmds-config")]
    pub mmds_config: Option<Box<MmdsConfig>>,
    /// Configurations for all net devices.
    #[serde(rename = "network-interfaces")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    pub vsock: Option<Box<Vsock>>,
}

impl FullVmConfiguration {
    pub fn new() -> Self {
        Default::default()
    }
}
