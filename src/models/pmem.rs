use camino::Utf8PathBuf;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Pmem {
    /// Identificator for this device.
    pub id: CompactString,
    /// Host level path for the virtio-pmem device to use as a backing file.
    pub path_on_host: Utf8PathBuf,
    /// Flag to make this device be the root device for VM boot. Setting this flag
    /// will fail if there is another device configured to be a root device already.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_device: Option<bool>,
    /// Flag to map backing file in read-only mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl Pmem {
    pub fn new(id: impl Into<CompactString>, path_on_host: impl Into<Utf8PathBuf>) -> Self {
        Self {
            id: id.into(),
            path_on_host: path_on_host.into(),
            root_device: None,
            read_only: None,
        }
    }
}
