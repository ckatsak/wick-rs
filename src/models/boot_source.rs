use camino::Utf8PathBuf;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Boot source descriptor.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BootSource {
    /// Kernel boot arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_args: Option<CompactString>,
    /// Host level path to the initrd image used to boot the guest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initrd_path: Option<Utf8PathBuf>,
    /// Host level path to the kernel image used to boot the guest
    pub kernel_image_path: Utf8PathBuf,
}

impl BootSource {
    #[inline]
    pub fn new(kernel_image_path: impl Into<Utf8PathBuf>) -> Self {
        Self {
            boot_args: None,
            initrd_path: None,
            kernel_image_path: kernel_image_path.into(),
        }
    }
}
