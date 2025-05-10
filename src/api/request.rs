use compact_str::{format_compact, CompactString};
use http_body_util::BodyExt;
use hyper::{
    header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE},
    http,
};
use hyper_util::client::legacy::connect::Connect;

use crate::api::{
    config::Config,
    error::{ApiError, Error},
};

pub(crate) struct Request {
    method: http::Method,
    path: CompactString,
    no_return_type: bool,
    serialized_body: Option<String>,
}

impl Request {
    pub fn new(method: http::Method, path: CompactString) -> Self {
        Request {
            method,
            path,
            serialized_body: None,
            no_return_type: false,
        }
    }

    pub fn with_body<B: ::serde::Serialize>(mut self, body: B) -> Self {
        self.serialized_body = Some(::serde_json::to_string(&body).unwrap());
        self
    }

    pub fn returns_nothing(mut self) -> Self {
        self.no_return_type = true;
        self
    }

    pub async fn execute<'a, C, U>(self, conf: &Config<C>) -> Result<U, Error>
    where
        C: Connect + Clone + Send + Sync,
        U: Sized + Send + 'a,
        for<'de> U: ::serde::Deserialize<'de>,
    {
        let uri = format_compact!("{}{}", conf.base_path, self.path)
            .parse::<::hyper::Uri>()
            .map_err(Error::UriError)?;
        let mut req_builder = ::hyper::Request::builder().uri(uri).method(self.method);

        let req_headers = req_builder.headers_mut().unwrap();
        let request = if let Some(body) = self.serialized_body {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            req_headers.insert(CONTENT_LENGTH, body.len().into());
            req_builder.body(body)
        } else {
            req_builder.body(String::new())
        }
        .map_err(Error::Http)?;

        let response = conf
            .client
            .request(request)
            .await
            .map_err(Error::HyperClient)?;

        if !response.status().is_success() {
            Err(Error::Api(ApiError {
                code: response.status(),
                body: response.into_body(),
            }))
        } else if self.no_return_type {
            // TODO:
            // - This is a hack; if there's no_ret_type, `U` is `()`, but `serde_json` fails
            //   to deserialize `""` into `()`, so deserialize "null" into it instead.
            // - An alternative option would be to require `U: Default`, and then return
            //   `U::default()` here instead, since `()` implements that, but then we'd need to
            //   `impl Default for` all models.
            Ok(::serde_json::from_str::<'_, U>("null").expect("serde null value"))
        } else {
            let collected = response.into_body().collect().await.map_err(Error::Hyper)?;
            ::serde_json::from_slice::<'_, U>(&collected.to_bytes()).map_err(Error::Serde)
        }
    }
}
