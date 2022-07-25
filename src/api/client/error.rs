use std::fmt;

use crate::api::ResponseContent;

/// The error type returned by [`MicrovmClient`] in cases of failure.
///
/// [`MicrovmClient`]: crate::MicrovmClient
#[derive(Debug)]
pub enum Error<T> {
    /// Wraps an underlying [`hyper::Error`].
    Hyper(hyper::Error),
    /// Wraps an underlying [`serde_json::Error`].
    Serde(serde_json::Error),
    /// Wraps an underlying [`std::io::Error`].
    Io(std::io::Error),
    /// Contains information about the response received by the server.
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Hyper(e) => ("hyper", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "{module} error: {e}")
    }
}

impl<T: fmt::Debug> std::error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Error::Hyper(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
