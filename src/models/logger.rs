use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Describes the configuration option for the logging capability.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Logger {
    /// Set the logging level.
    pub level: Option<Level>,
    /// Path to the named pipe or file for the human readable log output.
    pub log_path: PathBuf,
    /// Whether or not to output the level in the logs.
    pub show_level: Option<bool>,
    /// Whether or not to include the file path and line number of the log's origin.
    pub show_log_origin: Option<bool>,
}

impl Logger {
    /// Describes the configuration option for the logging capability.
    pub fn builder(log_path: impl AsRef<Path>) -> LoggerBuilder {
        LoggerBuilder {
            level: None,
            log_path: log_path.as_ref().to_path_buf(),
            show_level: None,
            show_log_origin: None,
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

#[derive(Clone, Debug)]
pub struct LoggerBuilder {
    level: Option<Level>,
    log_path: PathBuf,
    show_level: Option<bool>,
    show_log_origin: Option<bool>,
}

impl LoggerBuilder {
    pub fn level(&mut self, level: Level) -> &mut Self {
        self.level = Some(level);
        self
    }

    pub fn show_level(&mut self, show_level: bool) -> &mut Self {
        self.show_level = Some(show_level);
        self
    }

    pub fn show_log_origin(&mut self, show_log_origin: bool) -> &mut Self {
        self.show_log_origin = Some(show_log_origin);
        self
    }

    pub fn build(self) -> Logger {
        Logger {
            level: self.level,
            log_path: self.log_path,
            show_level: self.show_level,
            show_log_origin: self.show_log_origin,
        }
    }
}
