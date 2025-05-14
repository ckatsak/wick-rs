use std::path::{Path, PathBuf};

use compact_str::{CompactString, ToCompactString};
use hyper::http;
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;
use hyperlocal::UnixConnector;
use tracing::{instrument, Level};

use crate::{api::request::Request, models, Api, Error};

#[derive(Debug, Clone)]
pub struct Client {
    socket_path: PathBuf,
    client: HyperClient<UnixConnector, String>,
}

impl Client {
    /// Construct a default `Client` with a default
    /// [hyper `Client`]<code><[UnixConnector], [String]></code>.
    ///
    /// Use [`with_hyper_client`](Self::with_hyper_client) to construct a `Client` with a
    /// custom [hyper `Client`]<code><[UnixConnector], [String]></code>.
    ///
    /// # Example
    ///
    /// ```
    /// let fc_client = wick::Client::new("/tmp/fc.sock");
    /// ```
    ///
    /// [hyper `Client`]: HyperClient
    #[inline]
    pub fn new(socket_path: impl AsRef<Path>) -> Self {
        let client = HyperClient::builder(TokioExecutor::new()).build(UnixConnector);
        Self::with_hyper_client(socket_path, client)
    }

    /// Construct a new `Client` with a custom
    /// <code>[HyperClient]<[UnixConnector], [String]></code>.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::time::Duration;
    /// use hyper_util::client::legacy::Client as HyperClient;
    /// use hyper_util::rt::TokioExecutor;
    /// use hyperlocal::UnixConnector;
    /// use wick::Client;
    ///
    /// let hclient = HyperClient::builder(TokioExecutor::new())
    ///     .pool_idle_timeout(Duration::from_secs(30))
    ///     .build(UnixConnector);
    ///
    /// let fc_client = Client::with_hyper_client("/tmp/fc.sock", hclient);
    /// ```
    #[inline]
    pub fn with_hyper_client(
        socket_path: impl AsRef<Path>,
        client: HyperClient<UnixConnector, String>,
    ) -> Self {
        Self {
            socket_path: socket_path.as_ref().to_path_buf(),
            client,
        }
    }
}

impl Api for Client {
    #[instrument(level = Level::DEBUG, skip(self))]
    async fn create_snapshot(
        &self,
        create_params: models::SnapshotCreateParams,
    ) -> Result<(), Error> {
        const PATH: &str = "/snapshot/create";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(create_params)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn create_sync_action(
        &self,
        action_info: models::InstanceActionInfo,
    ) -> Result<(), Error> {
        const PATH: &str = "/actions";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(action_info)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn describe_balloon_config(&self) -> Result<models::Balloon, Error> {
        const PATH: &str = "/balloon";

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn describe_balloon_stats(&self) -> Result<models::BalloonStats, Error> {
        const PATH: &str = "/balloon/statistics";

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn describe_instance(&self) -> Result<models::InstanceInfo, Error> {
        const PATH: &str = "/";

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn get_export_vm_config(&self) -> Result<models::FullVmConfiguration, Error> {
        const PATH: &str = "/vm/config";

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn get_firecracker_version(&self) -> Result<models::FirecrackerVersion, Error> {
        const PATH: &str = "/version";

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn get_machine_configuration(&self) -> Result<models::MachineConfiguration, Error> {
        const PATH: &str = "/machine-config";

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn get_mmds(&self) -> Result<serde_json::Value, Error> {
        const PATH: &str = "/mmds";

        let path = PATH.to_compact_string();

        let req = Request::new(http::Method::GET, path);

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn load_snapshot(&self, load_params: models::SnapshotLoadParams) -> Result<(), Error> {
        const PATH: &str = "/snapshot/load";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(load_params)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn patch_balloon(&self, balloon_update: models::BalloonUpdate) -> Result<(), Error> {
        const PATH: &str = "/balloon";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(balloon_update)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn patch_balloon_stats_interval(
        &self,
        body: models::BalloonStatsUpdate,
    ) -> Result<(), Error> {
        const PATH: &str = "/balloon/statistics";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn patch_guest_drive_by_id(
        &self,
        drive_id: &str,
        body: models::PartialDrive,
    ) -> Result<(), Error> {
        const PATH: &str = "/drives/";

        let mut path = CompactString::with_capacity(PATH.len() + drive_id.len());
        path.push_str(PATH);
        path.push_str(drive_id);

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn patch_guest_network_interface_by_id(
        &self,
        iface_id: &str,
        body: models::PartialNetworkInterface,
    ) -> Result<(), Error> {
        const PATH: &str = "/network-interfaces/";

        let mut path = CompactString::with_capacity(PATH.len() + iface_id.len());
        path.push_str(PATH);
        path.push_str(iface_id);

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn patch_machine_configuration(
        &self,
        body: Option<models::MachineConfiguration>,
    ) -> Result<(), Error> {
        const PATH: &str = "/machine-config";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn patch_mmds(&self, body: Option<serde_json::Value>) -> Result<(), Error> {
        const PATH: &str = "/mmds";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn patch_vm(&self, vm: models::Vm) -> Result<(), Error> {
        const PATH: &str = "/vm";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PATCH, path);
        req = req.with_body(vm)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_balloon(&self, balloon: models::Balloon) -> Result<(), Error> {
        const PATH: &str = "/balloon";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(balloon)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_cpu_configuration(
        &self,
        cpu_config: Option<models::CpuConfig>,
    ) -> Result<(), Error> {
        const PATH: &str = "/cpu-config";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(cpu_config)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_entropy_device(&self, entropy_dev: models::EntropyDevice) -> Result<(), Error> {
        const PATH: &str = "/entropy";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(entropy_dev)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_guest_boot_source(&self, boot_source: models::BootSource) -> Result<(), Error> {
        const PATH: &str = "/boot-source";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(boot_source)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_guest_drive_by_id(
        &self,
        drive_id: &str,
        body: models::Drive,
    ) -> Result<(), Error> {
        const PATH: &str = "/drives/";

        let mut path = CompactString::with_capacity(PATH.len() + drive_id.len());
        path.push_str(PATH);
        path.push_str(drive_id);

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_guest_network_interface_by_id(
        &self,
        iface_id: &str,
        body: models::NetworkInterface,
    ) -> Result<(), Error> {
        const PATH: &str = "/network-interfaces/";

        let mut path = CompactString::with_capacity(PATH.len() + iface_id.len());
        path.push_str(PATH);
        path.push_str(iface_id);

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_guest_vsock(&self, vsock: models::Vsock) -> Result<(), Error> {
        const PATH: &str = "/vsock";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(vsock)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_logger(&self, logger: models::Logger) -> Result<(), Error> {
        const PATH: &str = "/logger";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(logger)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_machine_configuration(
        &self,
        body: Option<models::MachineConfiguration>,
    ) -> Result<(), Error> {
        const PATH: &str = "/machine-config";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_metrics(&self, metrics: models::Metrics) -> Result<(), Error> {
        const PATH: &str = "/metrics";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(metrics)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_mmds(&self, body: Option<serde_json::Value>) -> Result<(), Error> {
        const PATH: &str = "/mmds";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(body)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }

    #[instrument(level = Level::DEBUG, skip(self))]
    async fn put_mmds_config(&self, mmds_config: models::MmdsConfig) -> Result<(), Error> {
        const PATH: &str = "/mmds/config";

        let path = PATH.to_compact_string();

        let mut req = Request::new(http::Method::PUT, path);
        req = req.with_body(mmds_config)?;
        req = req.returns_nothing();

        req.execute(&self.socket_path, &self.client).await
    }
}
