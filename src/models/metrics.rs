use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

/// Describes the configuration option for the metrics capability.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    /// Path to the named pipe or file where the JSON-formatted metrics are flushed.
    pub metrics_path: Utf8PathBuf,
}

impl Metrics {
    #[inline]
    pub fn new(metrics_path: impl Into<Utf8PathBuf>) -> Self {
        Self {
            metrics_path: metrics_path.into(),
        }
    }
}
