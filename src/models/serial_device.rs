use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

/// The configuration of the serial device
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SerialDevice {
    /// Path to a file or named pipe on the host to which serial output should be written.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_out_path: Option<Utf8PathBuf>,
}
