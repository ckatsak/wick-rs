use std::{borrow::Borrow, sync::Arc};

use compact_str::{CompactString, ToCompactString};
use const_format::concatcp;
use hyper::http;

use crate::{
    api::{config::Config, request::Request},
    models, Api, Error,
};

pub const BASE_PATH: &str = "http://localhost";

pub struct Client {
    config: Arc<Config>,
}

impl Client {
    #[inline]
    pub fn new(config: Arc<Config>) -> Client {
        Client { config }
    }
}

impl Api for Client {
    async fn create_snapshot(&self, body: models::SnapshotCreateParams) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/snapshot/create");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn create_sync_action(&self, info: models::InstanceActionInfo) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/actions");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(info)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn describe_balloon_config(&self) -> Result<models::Balloon, Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/balloon");

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(self.config.borrow()).await
    }

    async fn describe_balloon_stats(&self) -> Result<models::BalloonStats, Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/balloon/statistics");

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(self.config.borrow()).await
    }

    async fn describe_instance(&self) -> Result<models::InstanceInfo, Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/");

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(self.config.borrow()).await
    }

    async fn get_export_vm_config(&self) -> Result<models::FullVmConfiguration, Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/vm/config");

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(self.config.borrow()).await
    }

    async fn get_firecracker_version(&self) -> Result<models::FirecrackerVersion, Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/version");

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(self.config.borrow()).await
    }

    async fn get_machine_configuration(&self) -> Result<models::MachineConfiguration, Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/machine-config");

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(self.config.borrow()).await
    }

    async fn get_mmds(&self) -> Result<serde_json::Value, Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/mmds");

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(self.config.borrow()).await
    }

    async fn load_snapshot(&self, body: models::SnapshotLoadParams) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/snapshot/load");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn patch_balloon(&self, body: models::BalloonUpdate) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/balloon");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn patch_balloon_stats_interval(
        &self,
        body: models::BalloonStatsUpdate,
    ) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/balloon/statistics");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn patch_guest_drive_by_id(
        &self,
        drive_id: &str,
        body: models::PartialDrive,
    ) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/drives/");

        let mut path = CompactString::with_capacity(PATH.len() + drive_id.len());
        path.push_str(PATH);
        path.push_str(drive_id);

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn patch_guest_network_interface_by_id(
        &self,
        iface_id: &str,
        body: models::PartialNetworkInterface,
    ) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/network-interfaces/");

        let mut path = CompactString::with_capacity(PATH.len() + iface_id.len());
        path.push_str(PATH);
        path.push_str(iface_id);

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn patch_machine_configuration(
        &self,
        body: Option<models::MachineConfiguration>,
    ) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/machine-config");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn patch_mmds(&self, body: Option<serde_json::Value>) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/mmds");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn patch_vm(&self, body: models::Vm) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/vm");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_balloon(&self, body: models::Balloon) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/balloon");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_cpu_configuration(&self, body: Option<models::CpuConfig>) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/cpu-config");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_entropy_device(&self, body: models::EntropyDevice) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/entropy");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_guest_boot_source(&self, body: models::BootSource) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/boot-source");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_guest_drive_by_id(
        &self,
        drive_id: &str,
        body: models::Drive,
    ) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/drives/");

        let mut path = CompactString::with_capacity(PATH.len() + drive_id.len());
        path.push_str(PATH);
        path.push_str(drive_id);

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_guest_network_interface_by_id(
        &self,
        iface_id: &str,
        body: models::NetworkInterface,
    ) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/network-interfaces/");

        let mut path = CompactString::with_capacity(PATH.len() + iface_id.len());
        path.push_str(PATH);
        path.push_str(iface_id);

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_guest_vsock(&self, body: models::Vsock) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/vsock");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_logger(&self, body: models::Logger) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/logger");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_machine_configuration(
        &self,
        body: Option<models::MachineConfiguration>,
    ) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/machine-config");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_metrics(&self, body: models::Metrics) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/metrics");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_mmds(&self, body: Option<serde_json::Value>) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/mmds");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }

    async fn put_mmds_config(&self, body: models::MmdsConfig) -> Result<(), Error> {
        const PATH: &str = concatcp!(BASE_PATH, "/mmds/config");

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(self.config.borrow()).await
    }
}
