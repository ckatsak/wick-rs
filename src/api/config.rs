use std::path::{Path, PathBuf};

use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use hyperlocal::UnixConnector;

#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) socket_path: PathBuf,
    pub(crate) client: Client<UnixConnector, String>,
}

impl Config {
    /// Construct a default `Config` with a default hyper [`Client`] using a
    /// [`UnixConnector`].
    ///
    /// Use [`with_client`](Config<T>::with_client) to construct a `Config` with a
    /// custom hyper [`Client`].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::apis::config::Config;
    /// let api_config = Config::new("/tmp/fc.sock");
    /// ```
    #[inline]
    pub fn new(socket_path: impl AsRef<Path>) -> Self {
        let client = Client::builder(TokioExecutor::new()).build(UnixConnector);
        Self::with_client(socket_path, client)
    }
}

impl Config {
    /// Construct a new `Config` with a custom hyper [`Client`].
    ///
    /// # Example
    ///
    /// ```
    /// # use core::time::Duration;
    /// # use crate::apis::config::Config;
    /// use hyper_util::client::legacy::Client;
    /// use hyper_util::rt::TokioExecutor;
    /// use hyperlocal::UnixConnector;
    ///
    /// let client = Client::builder(TokioExecutor::new())
    ///   .pool_idle_timeout(Duration::from_secs(30))
    ///   .build(UnixConnector);
    ///
    /// let api_config = Config::with_client("/tmp/fc.sock", client);
    /// ```
    #[inline]
    pub fn with_client(
        socket_path: impl AsRef<Path>,
        client: Client<UnixConnector, String>,
    ) -> Self {
        Self {
            socket_path: socket_path.as_ref().to_path_buf(),
            client,
        }
    }
}
