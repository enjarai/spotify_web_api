use super::error::ApiError;
use async_trait::async_trait;
use bytes::Bytes;
use http::{Response, request::Builder as RequestBuilder};
use std::error::Error;
use url::Url;

/// A trait representing a client which can communicate with a Spotify instance via REST.
pub trait RestClient {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for a REST v1 endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;
}

/// A trait representing a client which can communicate with a Spotify instance.
pub trait Client: RestClient {
    /// Send a REST query.
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client which can communicate with a Spotify instance.
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously.
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
