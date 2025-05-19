use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FullVmConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balloon: Option<Box<models::Balloon>>,
    /// Configurations for all block devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<models::Drive>>,
    #[serde(rename = "boot-source", skip_serializing_if = "Option::is_none")]
    pub boot_source: Option<Box<models::BootSource>>,
    #[serde(rename = "cpu-config", skip_serializing_if = "Option::is_none")]
    pub cpu_config: Option<Box<models::CpuConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger: Option<Box<models::Logger>>,
    #[serde(rename = "machine-config", skip_serializing_if = "Option::is_none")]
    pub machine_config: Option<Box<models::MachineConfiguration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Box<models::Metrics>>,
    #[serde(rename = "mmds-config", skip_serializing_if = "Option::is_none")]
    pub mmds_config: Option<Box<models::MmdsConfig>>,
    /// Configurations for all net devices.
    #[serde(rename = "network-interfaces", skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<models::NetworkInterface>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsock: Option<Box<models::Vsock>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy: Option<Box<models::EntropyDevice>>,
}
