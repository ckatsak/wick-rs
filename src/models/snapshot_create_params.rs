use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SnapshotCreateParams {
    /// Path to the file that will contain the guest memory.
    pub mem_file_path: PathBuf,
    /// Path to the file that will contain the microVM state.
    pub snapshot_path: PathBuf,
    /// Type of snapshot to create. It is optional and by default, a full snapshot is created.
    pub snapshot_type: Option<SnapshotType>,
    /// The microVM version for which we want to create the snapshot. It is optional and it
    /// defaults to the current version.
    pub version: Option<String>,
}

impl SnapshotCreateParams {
    pub fn builder(
        mem_file_path: impl AsRef<Path>,
        snapshot_path: impl AsRef<Path>,
    ) -> SnapshotCreateParamsBuilder {
        SnapshotCreateParamsBuilder {
            mem_file_path: mem_file_path.as_ref().to_path_buf(),
            snapshot_path: snapshot_path.as_ref().to_path_buf(),
            snapshot_type: None,
            version: None,
        }
    }

    ///// Path to the file that will contain the guest memory.
    //pub fn mem_file_path(&self) -> Cow<'_, Path> {
    //    Cow::Borrowed(&self.mem_file_path)
    //}

    ///// Path to the file that will contain the microVM state.
    //pub fn snapshot_path(&self) -> Cow<'_, Path> {
    //    Cow::Borrowed(&self.snapshot_path)
    //}

    ///// Type of snapshot to create. It is optional and by default, a full snapshot is created.
    //pub fn snapshot_type(&self) -> Option<SnapshotType> {
    //    self.snapshot_type
    //}

    ///// The microVM version for which we want to create the snapshot. It is optional and it
    ///// defaults to the current version.
    //pub fn version(&self) -> Option<Cow<'_, str>> {
    //    self.version.as_ref().map(|s| Cow::Borrowed(s.as_ref()))
    //}
}

/// Type of snapshot to create. It is optional and by default, a full snapshot is created.
#[derive(
    Clone, Copy, Debug, Eq, Default, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum SnapshotType {
    #[default]
    Full,
    Diff,
}

pub struct SnapshotCreateParamsBuilder {
    mem_file_path: PathBuf,
    snapshot_path: PathBuf,
    snapshot_type: Option<SnapshotType>,
    version: Option<String>,
}

impl SnapshotCreateParamsBuilder {
    /// Type of snapshot to create. It is optional and by default, a full snapshot is created.
    pub fn snapshot_type(&mut self, snapshot_type: SnapshotType) -> &mut Self {
        self.snapshot_type = Some(snapshot_type);
        self
    }

    /// The microVM version for which we want to create the snapshot. It is optional and it
    /// defaults to the current version.
    pub fn version(&mut self, version: impl AsRef<str>) -> &mut Self {
        self.version = Some(version.as_ref().to_owned());
        self
    }

    pub fn build(self) -> SnapshotCreateParams {
        SnapshotCreateParams {
            mem_file_path: self.mem_file_path,
            snapshot_path: self.snapshot_path,
            snapshot_type: self.snapshot_type,
            version: self.version,
        }
    }
}
