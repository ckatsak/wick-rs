use std::fmt::{self, Debug};

use hyper::http;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("API error")]
    Api(#[source] ApiError),

    #[error("HTTP error")]
    Http(#[source] http::Error),

    #[error("hyper error")]
    Hyper(#[source] ::hyper::Error),

    #[error("HTTP client error")]
    HyperClient(#[source] ::hyper_util::client::legacy::Error),

    #[error("(de)serialization error")]
    Serde(#[source] ::serde_json::Error),

    #[error("URI error")]
    UriError(#[source] http::uri::InvalidUri),
}

#[derive(::thiserror::Error)]
#[error("Server returned HTTP status: {code}")]
pub struct ApiError {
    pub code: ::hyper::StatusCode,
    pub body: ::hyper::body::Incoming,
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
            .field("code", &self.code)
            .field("body", &"::hyper::body::Incoming")
            .finish()
    }
}

impl From<(::hyper::StatusCode, ::hyper::body::Incoming)> for Error {
    #[inline]
    fn from((code, body): (::hyper::StatusCode, ::hyper::body::Incoming)) -> Self {
        Error::Api(ApiError { code, body })
    }
}
