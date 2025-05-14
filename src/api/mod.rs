pub mod client;
pub mod error;
mod request;

use std::future::Future;

use crate::{api::error::Error, models};

pub trait Api: Send + Sync {
    fn create_snapshot(
        &self,
        body: models::SnapshotCreateParams,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn create_sync_action(
        &self,
        info: models::InstanceActionInfo,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn describe_balloon_config(
        &self,
    ) -> impl Future<Output = Result<models::Balloon, Error>> + Send;

    fn describe_balloon_stats(
        &self,
    ) -> impl Future<Output = Result<models::BalloonStats, Error>> + Send;

    fn describe_instance(&self)
        -> impl Future<Output = Result<models::InstanceInfo, Error>> + Send;

    fn get_export_vm_config(
        &self,
    ) -> impl Future<Output = Result<models::FullVmConfiguration, Error>> + Send;

    fn get_firecracker_version(
        &self,
    ) -> impl Future<Output = Result<models::FirecrackerVersion, Error>> + Send;

    fn get_machine_configuration(
        &self,
    ) -> impl Future<Output = Result<models::MachineConfiguration, Error>> + Send;

    fn get_mmds(&self) -> impl Future<Output = Result<serde_json::Value, Error>> + Send;

    fn load_snapshot(
        &self,
        body: models::SnapshotLoadParams,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn patch_balloon(
        &self,
        body: models::BalloonUpdate,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn patch_balloon_stats_interval(
        &self,
        body: models::BalloonStatsUpdate,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn patch_guest_drive_by_id(
        &self,
        drive_id: &str,
        body: models::PartialDrive,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn patch_guest_network_interface_by_id(
        &self,
        iface_id: &str,
        body: models::PartialNetworkInterface,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn patch_machine_configuration(
        &self,
        body: Option<models::MachineConfiguration>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn patch_mmds(
        &self,
        body: Option<serde_json::Value>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn patch_vm(&self, body: models::Vm) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_balloon(&self, body: models::Balloon) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_cpu_configuration(
        &self,
        body: Option<models::CpuConfig>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_entropy_device(
        &self,
        body: models::EntropyDevice,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_guest_boot_source(
        &self,
        body: models::BootSource,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_guest_drive_by_id(
        &self,
        drive_id: &str,
        body: models::Drive,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_guest_network_interface_by_id(
        &self,
        iface_id: &str,
        body: models::NetworkInterface,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_guest_vsock(
        &self,
        body: models::Vsock,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_logger(&self, body: models::Logger) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_machine_configuration(
        &self,
        body: Option<models::MachineConfiguration>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_metrics(&self, body: models::Metrics) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_mmds(
        &self,
        body: Option<serde_json::Value>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn put_mmds_config(
        &self,
        body: models::MmdsConfig,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
