pub mod client;
mod error;

pub use error::CreateSnapshotError;
pub use error::CreateSyncActionError;
pub use error::DescribeBalloonConfigError;
pub use error::DescribeBalloonStatsError;
pub use error::DescribeInstanceError;
pub use error::GetExportVmConfigError;
pub use error::GetFirecrackerVersionError;
pub use error::GetMachineConfigurationError;
pub use error::GetMmdsError;
pub use error::LoadSnapshotError;
pub use error::PatchBalloonError;
pub use error::PatchBalloonStatsIntervalError;
pub use error::PatchGuestDriveByIdError;
pub use error::PatchGuestNetworkInterfaceByIdError;
pub use error::PatchMachineConfigurationError;
pub use error::PatchMmdsError;
pub use error::PatchVmError;
pub use error::PutBalloonError;
pub use error::PutGuestBootSourceError;
pub use error::PutGuestDriveByIdError;
pub use error::PutGuestNetworkInterfaceByIdError;
pub use error::PutGuestVsockError;
pub use error::PutLoggerError;
pub use error::PutMachineConfigurationError;
pub use error::PutMetricsError;
pub use error::PutMmdsConfigError;
pub use error::PutMmdsError;

use std::borrow::Cow;

use hyper::StatusCode;

/// Information about the received HTTP response.
#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    status: StatusCode,
    content: String,
    entity: Option<T>,
}

impl<T> ResponseContent<T> {
    /// Returns the HTTP status code of the received response.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Returns the content of the received response as a string.
    pub fn content(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.content)
    }

    /// Returns the actual [(typed) kind of failure](crate::api#enums) parsed from the body of the
    /// received HTTP response.
    pub fn entity(&self) -> Option<&T> {
        self.entity.as_ref()
    }
}

fn urlencode(s: impl AsRef<str>) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}
