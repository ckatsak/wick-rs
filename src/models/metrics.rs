use serde::{Deserialize, Serialize};

/// Describes the configuration option for the metrics capability.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Metrics {
    /// Path to the named pipe or file where the JSON-formatted metrics are flushed.
    pub metrics_path: String,
}

impl Metrics {
    /// Describes the configuration option for the metrics capability.
    pub fn new(metrics_path: String) -> Self {
        Self { metrics_path }
    }
}
