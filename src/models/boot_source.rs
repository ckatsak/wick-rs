use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Boot source descriptor.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BootSource {
    /// Kernel boot arguments
    pub boot_args: Option<String>,
    /// Host level path to the initrd image used to boot the guest
    pub initrd_path: Option<PathBuf>,
    /// Host level path to the kernel image used to boot the guest
    pub kernel_image_path: PathBuf,
}

impl BootSource {
    pub fn builder(kernel_image_path: impl AsRef<PathBuf>) -> BootSourceBuilder {
        BootSourceBuilder {
            boot_args: None,
            initrd_path: None,
            kernel_image_path: kernel_image_path.as_ref().to_path_buf(),
        }
    }

    /// Create a new boot source descriptor with kernel boot arguments.
    pub fn with_boot_args(kernel_image_path: impl AsRef<Path>, boot_args: impl AsRef<str>) -> Self {
        Self {
            boot_args: Some(boot_args.as_ref().to_owned()),
            kernel_image_path: kernel_image_path.as_ref().to_path_buf(),
            initrd_path: None,
        }
    }

    ///// Kernel boot arguments
    //pub fn boot_args(&self) -> Option<Cow<'_, str>> {
    //    self.boot_args.as_ref().map(|s| Cow::Borrowed(s.as_ref()))
    //}

    ///// Host level path to the initrd image used to boot the guest
    //pub fn initrd_path(&self) -> Option<Cow<'_, Path>> {
    //    self.initrd_path.as_ref().map(|p| Cow::Borrowed(p.as_ref()))
    //}

    ///// Host level path to the kernel image used to boot the guest
    //pub fn kernel_image_path(&self) -> Cow<'_, Path> {
    //    Cow::Borrowed(&self.kernel_image_path)
    //}
}

#[derive(Debug, Clone)]
pub struct BootSourceBuilder {
    boot_args: Option<String>,
    initrd_path: Option<PathBuf>,
    kernel_image_path: PathBuf,
}

impl BootSourceBuilder {
    /// Kernel boot arguments
    pub fn boot_args(&mut self, boot_args: String) -> &mut Self {
        self.boot_args = Some(boot_args);
        self
    }

    /// Host level path to the initrd image used to boot the guest
    pub fn initrd_path(&mut self, initrd_path: PathBuf) -> &mut Self {
        self.initrd_path = Some(initrd_path);
        self
    }

    pub fn build(self) -> BootSource {
        BootSource {
            boot_args: self.boot_args,
            initrd_path: self.initrd_path,
            kernel_image_path: self.kernel_image_path,
        }
    }
}
