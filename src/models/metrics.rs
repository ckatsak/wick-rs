use serde::{Deserialize, Serialize};

/// Describes the configuration option for the metrics capability.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    /// Path to the named pipe or file where the JSON-formatted metrics are flushed.
    #[serde(rename = "metrics_path")]
    pub metrics_path: String,
}

impl Metrics {
    /// Describes the configuration option for the metrics capability.
    pub fn new(metrics_path: String) -> Self {
        Self { metrics_path }
    }
}
