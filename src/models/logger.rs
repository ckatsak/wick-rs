use camino::Utf8PathBuf;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Describes the configuration option for the logging capability.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Logger {
    /// Set the level. The possible values are case-insensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    /// Path to the named pipe or file for the human readable log output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_path: Option<Utf8PathBuf>,
    /// Whether or not to output the level in the logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_level: Option<bool>,
    /// Whether or not to include the file path and line number of the log's origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_log_origin: Option<bool>,
    /// The module path to filter log messages by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<CompactString>,
}

/// Set the level. The possible values are case-insensitive.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Level {
    #[default]
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}
