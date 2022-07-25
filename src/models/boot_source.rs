use serde::{Deserialize, Serialize};

/// Boot source descriptor.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BootSource {
    /// Kernel boot arguments
    pub boot_args: Option<String>,
    /// Host level path to the initrd image used to boot the guest
    pub initrd_path: Option<String>,
    /// Host level path to the kernel image used to boot the guest
    pub kernel_image_path: String,
}

impl BootSource {
    /// Boot source descriptor.
    pub fn new(kernel_image_path: String) -> Self {
        Self {
            kernel_image_path,
            ..Default::default()
        }
    }
}
