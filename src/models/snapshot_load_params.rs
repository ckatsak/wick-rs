use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::models::MemoryBackend;

/// Defines the configuration used for handling snapshot resume. Exactly one of the two `mem_*`
/// fields must be present in the body of the request.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SnapshotLoadParams {
    /// Enable support for incremental (diff) snapshots by tracking dirty guest pages.
    pub enable_diff_snapshots: Option<bool>,
    /// Path to the file that contains the guest memory to be loaded. This parameter has been
    /// deprecated and is only allowed if `mem_backend` is not present.
    pub mem_file_path: Option<PathBuf>,
    pub mem_backend: Option<MemoryBackend>,
    /// Path to the file that contains the microVM state to be loaded.
    pub snapshot_path: PathBuf,
    /// When set to true, the vm is also resumed if the snapshot load is successful.
    pub resume_vm: Option<bool>,
}

impl SnapshotLoadParams {
    /// Defines the configuration used for handling snapshot resume. Exactly one of the two `mem_*`
    /// fields must be present in the body of the request.
    pub fn builder(snapshot_path: impl AsRef<Path>) -> SnapshotLoadParamsBuilder {
        SnapshotLoadParamsBuilder {
            enable_diff_snapshots: None,
            mem_file_path: None,
            mem_backend: None,
            snapshot_path: snapshot_path.as_ref().to_path_buf(),
            resume_vm: None,
        }
    }

    ///// Enable support for incremental (diff) snapshots by tracking dirty guest pages.
    //pub fn enable_diff_snapshots(&self) -> Option<bool> {
    //    self.enable_diff_snapshots
    //}

    ///// Path to the file that contains the guest memory to be loaded. This parameter has been
    ///// deprecated and is only allowed if `mem_backend` is not present.
    //pub fn mem_file_path(&self) -> Option<Cow<'_, Path>> {
    //    self.mem_file_path
    //        .as_ref()
    //        .map(|p| Cow::Borrowed(p.as_ref()))
    //}

    //pub fn mem_backend(&self) -> Option<&MemoryBackend> {
    //    self.mem_backend.as_ref()
    //}

    ///// Path to the file that contains the microVM state to be loaded.
    //pub fn snapshot_path(&self) -> Cow<'_, Path> {
    //    Cow::Borrowed(&self.snapshot_path)
    //}

    ///// When set to true, the vm is also resumed if the snapshot load is successful.
    //pub fn resume_vm(&self) -> Option<bool> {
    //    self.resume_vm
    //}
}

pub struct SnapshotLoadParamsBuilder {
    enable_diff_snapshots: Option<bool>,
    mem_file_path: Option<PathBuf>,
    mem_backend: Option<MemoryBackend>,
    snapshot_path: PathBuf,
    resume_vm: Option<bool>,
}

impl SnapshotLoadParamsBuilder {
    /// Enable support for incremental (diff) snapshots by tracking dirty guest pages.
    pub fn enable_diff_snapshots(&mut self, enable_diff_snapshots: bool) -> &mut Self {
        self.enable_diff_snapshots = Some(enable_diff_snapshots);
        self
    }

    /// Path to the file that contains the guest memory to be loaded. This parameter has been
    /// deprecated and is only allowed if `mem_backend` is not present.
    pub fn mem_file_path(&mut self, mem_file_path: impl AsRef<Path>) -> &mut Self {
        self.mem_file_path = Some(mem_file_path.as_ref().to_path_buf());
        self
    }

    pub fn mem_backend(&mut self, mem_backend: MemoryBackend) -> &mut Self {
        self.mem_backend = Some(mem_backend);
        self
    }

    /// When set to true, the vm is also resumed if the snapshot load is successful.
    pub fn resume_vm(&mut self, resume_vm: bool) -> &mut Self {
        self.resume_vm = Some(resume_vm);
        self
    }

    pub fn build(self) -> SnapshotLoadParams {
        SnapshotLoadParams {
            enable_diff_snapshots: self.enable_diff_snapshots,
            mem_file_path: self.mem_file_path,
            mem_backend: self.mem_backend,
            snapshot_path: self.snapshot_path,
            resume_vm: self.resume_vm,
        }
    }
}
