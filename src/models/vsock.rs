use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Defines a vsock device, backed by a set of Unix Domain Sockets, on the host side. For
/// host-initiated connections, Firecracker will be listening on the Unix socket identified by the
/// path `uds_path`. Firecracker will create this socket, bind and listen on it. Host-initiated
/// connections will be performed by connection to this socket and issuing a connection forwarding
/// request to the desired guest-side vsock port (i.e. `CONNECT 52\\n`, to connect to port 52). For
/// guest-initiated connections, Firecracker will expect host software to be bound and listening on
/// Unix sockets at `uds_path_<PORT>`. E.g. \"/path/to/host_vsock.sock_52\" for port number 52.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Vsock {
    /// Guest Vsock CID
    pub guest_cid: i32,
    /// Path to UNIX domain socket, used to proxy vsock connections.
    pub uds_path: PathBuf,
    /// This parameter has been deprecated since v1.1.0.
    pub vsock_id: Option<String>,
}

impl Vsock {
    /// Defines a vsock device, backed by a set of Unix Domain Sockets, on the host side. For
    /// host-initiated connections, Firecracker will be listening on the Unix socket identified by
    /// the path `uds_path`. Firecracker will create this socket, bind and listen on it.
    /// Host-initiated connections will be performed by connection to this socket and issuing a
    /// connection forwarding request to the desired guest-side vsock port (i.e. `CONNECT 52\\n`,
    /// to connect to port 52). For guest-initiated connections, Firecracker will expect host
    /// software to be bound and listening on Unix sockets at `uds_path_<PORT>`. E.g.
    /// \"/path/to/host_vsock.sock_52\" for port number 52.
    pub fn new(guest_cid: i32, uds_path: impl AsRef<Path>) -> Self {
        Self {
            guest_cid,
            uds_path: uds_path.as_ref().to_path_buf(),
            vsock_id: None,
        }
    }

    /// Defines a vsock device, backed by a set of Unix Domain Sockets, on the host side. For
    /// host-initiated connections, Firecracker will be listening on the Unix socket identified by
    /// the path `uds_path`. Firecracker will create this socket, bind and listen on it.
    /// Host-initiated connections will be performed by connection to this socket and issuing a
    /// connection forwarding request to the desired guest-side vsock port (i.e. `CONNECT 52\\n`,
    /// to connect to port 52). For guest-initiated connections, Firecracker will expect host
    /// software to be bound and listening on Unix sockets at `uds_path_<PORT>`. E.g.
    /// \"/path/to/host_vsock.sock_52\" for port number 52.
    pub fn with_vsock_id(
        guest_cid: i32,
        uds_path: impl AsRef<Path>,
        vsock_id: impl AsRef<str>,
    ) -> Self {
        Self {
            guest_cid,
            uds_path: uds_path.as_ref().to_path_buf(),
            vsock_id: Some(vsock_id.as_ref().to_owned()),
        }
    }
}
