use serde::{Deserialize, Serialize};

use crate::models::{
    Balloon, BootSource, Drive, Logger, MachineConfiguration, Metrics, MmdsConfig,
    NetworkInterface, Vsock,
};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FullVmConfiguration {
    pub balloon: Option<Balloon>,
    /// Configurations for all block devices.
    pub drives: Option<Vec<Drive>>,
    #[serde(rename = "boot-source")]
    pub boot_source: Option<BootSource>,
    pub logger: Option<Logger>,
    #[serde(rename = "machine-config")]
    pub machine_config: Option<MachineConfiguration>,
    pub metrics: Option<Metrics>,
    #[serde(rename = "mmds-config")]
    pub mmds_config: Option<MmdsConfig>,
    /// Configurations for all net devices.
    #[serde(rename = "network-interfaces")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    pub vsock: Option<Vsock>,
}
