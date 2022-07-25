mod error;
pub use error::Error;

use std::path::{Path, PathBuf};

use hyper::{
    body,
    header::{ACCEPT, CONTENT_TYPE},
    Body, Client, Request, StatusCode,
};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use tracing::{debug, trace};

use crate::{
    api::{
        error::{
            CreateSnapshotError, CreateSyncActionError, DescribeBalloonConfigError,
            DescribeBalloonStatsError, DescribeInstanceError, GetExportVmConfigError,
            GetFirecrackerVersionError, GetMachineConfigurationError, GetMmdsError,
            LoadSnapshotError, PatchBalloonError, PatchBalloonStatsIntervalError,
            PatchGuestDriveByIdError, PatchGuestNetworkInterfaceByIdError,
            PatchMachineConfigurationError, PatchMmdsError, PatchVmError, PutBalloonError,
            PutGuestBootSourceError, PutGuestDriveByIdError, PutGuestNetworkInterfaceByIdError,
            PutGuestVsockError, PutLoggerError, PutMachineConfigurationError, PutMetricsError,
            PutMmdsConfigError, PutMmdsError,
        },
        urlencode, ResponseContent,
    },
    models::{
        Balloon, BalloonStats, BalloonStatsUpdate, BalloonUpdate, BootSource, Drive,
        FirecrackerVersion, FullVmConfiguration, InstanceActionInfo, InstanceInfo, Logger,
        MachineConfiguration, Metrics, MmdsConfig, NetworkInterface, PartialDrive,
        PartialNetworkInterface, SnapshotCreateParams, SnapshotLoadParams, Vm, Vsock,
    },
};

const APPLICATION_JSON: &str = "application/json";

/// A client for the MicroVM listening on a specific Unix socket (configured during instantiation).
#[derive(Debug, Clone)]
pub struct MicrovmClient {
    sock_path: PathBuf,
    client: Client<UnixConnector>,
}

impl MicrovmClient {
    /// Creates a new client for the MicroVM listening on the Unix socket at the provided path.
    pub fn new(socket_path: impl AsRef<Path>) -> Self {
        Self {
            sock_path: socket_path.as_ref().to_owned(),
            client: Client::unix(),
        }
    }

    /// Returns a reference to the path of the Unix socket that this `MicrovmClient` has been
    /// configured to attempt to connect to.
    pub fn socket_path(&self) -> &Path {
        self.sock_path.as_ref()
    }

    /// Creates a snapshot of the microVM state. The microVM should be in the `Paused` state.
    pub async fn create_snapshot(
        &self,
        body: SnapshotCreateParams,
    ) -> Result<(), Error<CreateSnapshotError>> {
        const ENDPOINT: &str = "/snapshot/create";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Creates a synchronous action.
    pub async fn create_sync_action(
        &self,
        info: InstanceActionInfo,
    ) -> Result<(), Error<CreateSyncActionError>> {
        const ENDPOINT: &str = "/actions";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&info)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Returns the current balloon device configuration.
    pub async fn describe_balloon_config(
        &self,
    ) -> Result<Balloon, Error<DescribeBalloonConfigError>> {
        const ENDPOINT: &str = "/balloon";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::get(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(Body::empty())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::OK => {
                serde_json::from_slice(body::to_bytes(resp.into_body()).await?.as_ref())
                    .map_err(Error::from)
            }
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Returns the latest balloon device statistics, only if enabled pre-boot.
    pub async fn describe_balloon_stats(
        &self,
    ) -> Result<BalloonStats, Error<DescribeBalloonStatsError>> {
        const ENDPOINT: &str = "/balloon/statistics";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::get(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(Body::empty())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::OK => {
                serde_json::from_slice(body::to_bytes(resp.into_body()).await?.as_ref())
                    .map_err(Error::from)
            }
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Returns general information about an instance.
    pub async fn describe_instance(&self) -> Result<InstanceInfo, Error<DescribeInstanceError>> {
        const ENDPOINT: &str = "/";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::get(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(Body::empty())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::OK => {
                serde_json::from_slice(body::to_bytes(resp.into_body()).await?.as_ref())
                    .map_err(Error::from)
            }
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Gets the full VM configuration
    ///
    /// Gets configuration for all VM resources. If the VM is restored from a snapshot, the
    /// boot-source, machine-config.smt and machine-config.cpu_template will be empty.
    pub async fn get_export_vm_config(
        &self,
    ) -> Result<FullVmConfiguration, Error<GetExportVmConfigError>> {
        const ENDPOINT: &str = "/vm/config";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::get(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(Body::empty())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::OK => {
                serde_json::from_slice(body::to_bytes(resp.into_body()).await?.as_ref())
                    .map_err(Error::from)
            }
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Gets the Firecracker version.
    pub async fn get_firecracker_version(
        &self,
    ) -> Result<FirecrackerVersion, Error<GetFirecrackerVersionError>> {
        const ENDPOINT: &str = "/version";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::get(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(Body::empty())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::OK => {
                serde_json::from_slice(body::to_bytes(resp.into_body()).await?.as_ref())
                    .map_err(Error::from)
            }
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Gets the machine configuration of the VM
    ///
    /// Gets the machine configuration of the VM. When called before the PUT operation, it will
    /// return the default values for the vCPU count (=1), memory size (=128 MiB). By default SMT
    /// is disabled and there is no CPU Template.
    pub async fn get_machine_configuration(
        &self,
    ) -> Result<MachineConfiguration, Error<GetMachineConfigurationError>> {
        const ENDPOINT: &str = "/machine-config";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::get(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(Body::empty())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::OK => {
                serde_json::from_slice(body::to_bytes(resp.into_body()).await?.as_ref())
                    .map_err(Error::from)
            }
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Get the MMDS data store.
    pub async fn get_mmds(&self) -> Result<serde_json::Value, Error<GetMmdsError>> {
        const ENDPOINT: &str = "/mmds";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::get(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(Body::empty())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::OK => {
                serde_json::from_slice(body::to_bytes(resp.into_body()).await?.as_ref())
                    .map_err(Error::from)
            }
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Loads a snapshot. Pre-boot only.
    ///
    /// Loads the microVM state from a snapshot. Only accepted on a fresh Firecracker process
    /// (before configuring any resource other than the Logger and Metrics).
    pub async fn load_snapshot(
        &self,
        body: SnapshotLoadParams,
    ) -> Result<(), Error<LoadSnapshotError>> {
        const ENDPOINT: &str = "/snapshot/load";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Updates a balloon device
    ///
    /// Updates an existing balloon device, before or after machine startup. Will fail if update is
    /// not possible.
    pub async fn patch_balloon(&self, body: BalloonUpdate) -> Result<(), Error<PatchBalloonError>> {
        const ENDPOINT: &str = "/balloon";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::patch(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Updates a balloon device statistics polling interval
    ///
    /// Updates an existing balloon device statistics interval, before or after machine startup.
    /// Will fail if update is not possible.
    pub async fn patch_balloon_stats_interval(
        &self,
        body: BalloonStatsUpdate,
    ) -> Result<(), Error<PatchBalloonStatsIntervalError>> {
        const ENDPOINT: &str = "/balloon/statistics";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::patch(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Updates the properties of a drive. Post-boot only.
    ///
    /// Updates the properties of the drive with the ID specified by drive_id path parameter. Will
    /// fail if update is not possible.
    pub async fn patch_guest_drive_by_id(
        &self,
        drive_id: &str,
        body: PartialDrive,
    ) -> Result<(), Error<PatchGuestDriveByIdError>> {
        let uri = Uri::new(&self.sock_path, &format!("/drives/{}", urlencode(drive_id)));
        debug!("URI: {uri:?}");

        let req = Request::patch(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Updates the rate limiters applied to a network interface. Post-boot-only.
    pub async fn patch_guest_network_interface_by_id(
        &self,
        iface_id: &str,
        body: PartialNetworkInterface,
    ) -> Result<(), Error<PatchGuestNetworkInterfaceByIdError>> {
        let uri = Uri::new(
            &self.sock_path,
            &format!("/network-interfaces/{}", urlencode(iface_id)),
        );
        debug!("URI: {uri:?}");

        let req = Request::patch(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Partially updates the Machine Configuration of the VM. Pre-boot only.
    ///
    /// Partially updates the Virtual Machine Configuration with the specified input. If any of the
    /// parameters has an incorrect value, the whole update fails.
    pub async fn patch_machine_configuration(
        &self,
        body: Option<MachineConfiguration>,
    ) -> Result<(), Error<PatchMachineConfigurationError>> {
        const ENDPOINT: &str = "/machine-config";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::patch(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Updates the MMDS data store.
    pub async fn patch_mmds(
        &self,
        body: Option<serde_json::Value>,
    ) -> Result<(), Error<PatchMmdsError>> {
        const ENDPOINT: &str = "/mmds";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::patch(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Updates the microVM state
    ///
    /// Sets the desired state (Paused or Resumed) for the microVM.
    pub async fn patch_vm(&self, body: Vm) -> Result<(), Error<PatchVmError>> {
        const ENDPOINT: &str = "/vm";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::patch(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Creates or updates a balloon device
    ///
    /// Creates a new balloon device if one does not already exist, otherwise updates it, before
    /// machine startup. This will fail after machine startup. Will fail if update is not possible.
    pub async fn put_balloon(&self, body: Balloon) -> Result<(), Error<PutBalloonError>> {
        const ENDPOINT: &str = "/balloon";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Creates or updates the boot source. Pre-boot only.
    ///
    /// Creates new boot source if one does not already exist, otherwise updates it. Will fail if
    /// update is not possible.
    pub async fn put_guest_boot_source(
        &self,
        body: BootSource,
    ) -> Result<(), Error<PutGuestBootSourceError>> {
        const ENDPOINT: &str = "/boot-source";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Creates or updates a drive. Pre-boot only.
    ///
    /// Creates new drive with ID specified by drive_id path parameter. If a drive with the
    /// specified ID already exists, updates its state based on new input. Will fail if update is
    /// not possible.
    pub async fn put_guest_drive_by_id(
        &self,
        drive_id: &str,
        body: Drive,
    ) -> Result<(), Error<PutGuestDriveByIdError>> {
        let uri = Uri::new(&self.sock_path, &format!("/drives/{}", urlencode(drive_id)));
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Creates a network interface. Pre-boot only.
    ///
    /// Creates new network interface with ID specified by iface_id path parameter.
    pub async fn put_guest_network_interface_by_id(
        &self,
        iface_id: &str,
        body: NetworkInterface,
    ) -> Result<(), Error<PutGuestNetworkInterfaceByIdError>> {
        let uri = Uri::new(
            &self.sock_path,
            &format!("/network-interfaces/{}", urlencode(iface_id)),
        );
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    ///  Creates/updates a vsock device. Pre-boot only.
    ///
    /// The first call creates the device with the configuration specified in body. Subsequent
    /// calls will update the device configuration. May fail if update is not possible.
    pub async fn put_guest_vsock(&self, body: Vsock) -> Result<(), Error<PutGuestVsockError>> {
        const ENDPOINT: &str = "/vsock";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Initializes the logger by specifying a named pipe or a file for the logs output.
    pub async fn put_logger(&self, body: Logger) -> Result<(), Error<PutLoggerError>> {
        const ENDPOINT: &str = "/logger";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Updates the Machine Configuration of the VM. Pre-boot only.
    ///
    /// Updates the Virtual Machine Configuration with the specified input. Firecracker starts with
    /// default values for vCPU count (=1) and memory size (=128 MiB). The vCPU count is restricted
    /// to the [1, 32] range. With SMT enabled, the vCPU count is required to be either 1 or an
    /// even number in the range. otherwise there are no restrictions regarding the vCPU count. If
    /// any of the parameters has an incorrect value, the whole update fails. All parameters that
    /// are optional and are not specified are set to their default values (smt = false,
    /// track_dirty_pages = false, cpu_template = None).
    pub async fn put_machine_configuration(
        &self,
        body: Option<MachineConfiguration>,
    ) -> Result<(), Error<PutMachineConfigurationError>> {
        const ENDPOINT: &str = "/machine-config";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Initializes the metrics system by specifying a named pipe or a file for the metrics output.
    pub async fn put_metrics(&self, body: Metrics) -> Result<(), Error<PutMetricsError>> {
        const ENDPOINT: &str = "/metrics";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Create a MMDS (MicroVM Metadata Service) data store.
    pub async fn put_mmds(
        &self,
        body: Option<serde_json::Value>,
    ) -> Result<(), Error<PutMmdsError>> {
        const ENDPOINT: &str = "/mmds";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }

    /// Set MMDS configuration. Pre-boot only.
    ///
    /// Configures MMDS version, IPv4 address used by the MMDS network stack and interfaces that
    /// allow MMDS requests.
    pub async fn put_mmds_config(&self, body: MmdsConfig) -> Result<(), Error<PutMmdsConfigError>> {
        const ENDPOINT: &str = "/mmds/config";
        let uri = Uri::new(&self.sock_path, ENDPOINT);
        debug!("URI: {uri:?}");

        let req = Request::put(uri)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_string(&body)?.into())
            .unwrap();
        trace!("Request: {req:#?}");

        let resp = self.client.request(req).await?;
        trace!("Response: {resp:#?}");

        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _code => {
                let (parts, body) = resp.into_parts();
                let body_bytes = body::to_bytes(body).await?;
                Err(Error::ResponseError(ResponseContent {
                    status: parts.status,
                    content: String::from_utf8_lossy(body_bytes.as_ref()).to_string(),
                    entity: serde_json::from_slice(&body_bytes).ok(),
                }))
            }
        }
    }
}
