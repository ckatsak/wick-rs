use hyper_util::{
    client::legacy::{connect::Connect, Client},
    rt::TokioExecutor,
};
use hyperlocal::UnixConnector;

pub struct Config<C = UnixConnector>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    pub base_path: &'static str,
    pub client: Client<C, String>,
}

impl Config<UnixConnector> {
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
    /// let api_config = Config::new();
    /// ```
    #[inline]
    pub fn new() -> Config<UnixConnector> {
        Config::default()
    }
}

impl<C> Config<C>
where
    C: Connect + Clone + Send + Sync,
{
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
    ///   .build(UnixConnector::default());
    ///
    /// let api_config = Config::with_client(client);
    /// ```
    #[inline]
    pub fn with_client(client: Client<C, String>) -> Config<C> {
        Config {
            base_path: "http://localhost",
            client,
        }
    }
}

impl Default for Config<UnixConnector> {
    fn default() -> Self {
        let client = Client::builder(TokioExecutor::new()).build(Default::default());
        Config::with_client(client)
    }
}
