use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullVmConfiguration {
    #[serde(rename = "balloon", skip_serializing_if = "Option::is_none")]
    pub balloon: Option<Box<models::Balloon>>,
    /// Configurations for all block devices.
    #[serde(rename = "drives", skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<models::Drive>>,
    #[serde(rename = "boot-source", skip_serializing_if = "Option::is_none")]
    pub boot_source: Option<Box<models::BootSource>>,
    #[serde(rename = "cpu-config", skip_serializing_if = "Option::is_none")]
    pub cpu_config: Option<Box<models::CpuConfig>>,
    #[serde(rename = "logger", skip_serializing_if = "Option::is_none")]
    pub logger: Option<Box<models::Logger>>,
    #[serde(rename = "machine-config", skip_serializing_if = "Option::is_none")]
    pub machine_config: Option<Box<models::MachineConfiguration>>,
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Box<models::Metrics>>,
    #[serde(rename = "mmds-config", skip_serializing_if = "Option::is_none")]
    pub mmds_config: Option<Box<models::MmdsConfig>>,
    /// Configurations for all net devices.
    #[serde(rename = "network-interfaces", skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<models::NetworkInterface>>,
    #[serde(rename = "vsock", skip_serializing_if = "Option::is_none")]
    pub vsock: Option<Box<models::Vsock>>,
    #[serde(rename = "entropy", skip_serializing_if = "Option::is_none")]
    pub entropy: Option<Box<models::EntropyDevice>>,
}

impl FullVmConfiguration {
    pub fn new() -> Self {
        Default::default()
    }
}
