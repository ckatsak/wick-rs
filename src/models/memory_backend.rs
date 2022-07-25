use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryBackend {
    pub backend_type: BackendType,
    /// Based on 'backend_type' it is either 1) Path to the file that contains the guest memory to
    /// be loaded 2) Path to the UDS where a process is listening for a UFFD initialization control
    /// payload and open file descriptor that it can use to serve this process's guest memory page
    /// faults.
    pub backend_path: String,
}

impl MemoryBackend {
    pub fn new(backend_type: BackendType, backend_path: String) -> Self {
        Self {
            backend_type,
            backend_path,
        }
    }
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum BackendType {
    #[default]
    File,
    Uffd,
}
