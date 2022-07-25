use serde::{Deserialize, Serialize};

/// Describes the configuration option for the logging capability.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Logger {
    /// Set the logging level.
    pub level: Option<Level>,
    /// Path to the named pipe or file for the human readable log output.
    pub log_path: String,
    /// Whether or not to output the level in the logs.
    pub show_level: Option<bool>,
    /// Whether or not to include the file path and line number of the log's origin.
    pub show_log_origin: Option<bool>,
}

impl Logger {
    /// Describes the configuration option for the logging capability.
    pub fn new(log_path: String) -> Self {
        Self {
            log_path,
            ..Default::default()
        }
    }
}

/// Logging level.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Level {
    #[default]
    Error,
    Warning,
    Info,
    Debug,
}
