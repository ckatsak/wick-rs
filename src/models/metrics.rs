use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Describes the configuration option for the metrics capability.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Metrics {
    /// Path to the named pipe or file where the JSON-formatted metrics are flushed.
    pub metrics_path: PathBuf,
}

impl Metrics {
    /// Describes the configuration option for the metrics capability.
    pub fn new(metrics_path: impl AsRef<Path>) -> Self {
        Self {
            metrics_path: metrics_path.as_ref().to_path_buf(),
        }
    }

    ///// Path to the named pipe or file where the JSON-formatted metrics are flushed.
    //pub fn metrics_path(&self) -> Cow<'_, Path> {
    //    Cow::Borrowed(&self.metrics_path)
    //}
}
